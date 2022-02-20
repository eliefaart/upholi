use super::entities::photo::Photo;
use super::entities::share::Share;
use super::entities::user::User;
use crate::database;
use crate::database::SortField;
use crate::error::*;
use async_once::AsyncOnce;
use bson::doc;
use futures::TryStreamExt;
use lazy_static::lazy_static;
use mongodb::{options::ClientOptions, Client};
use upholi_lib::http::request::{FindEntity, FindSharesFilter};
use upholi_lib::http::response::PhotoMinimal;

lazy_static! {
	/// A reference to the database that can be used to execute queries etc
	static ref DATABASE: AsyncOnce<mongodb::Database> = AsyncOnce::new(async {
		let client_options = ClientOptions::parse(&crate::SETTINGS.database.connection_string)
			.await.expect("Failed to parse database connection string");

		let client = Client::with_options(client_options)
			.expect("Failed to initialize database client");

		let database = client.default_database()
			.expect("No default database found in connection string");

		if let Err(error) = initialize(&database).await {
			println!("Error preparing database: {:?}", error);
		}

		database
	});
}

/// Initialize database by setting some indexes if needed
async fn initialize(database: &mongodb::Database) -> Result<()> {
	create_index(database, crate::database::COLLECTION_SESSIONS, "id").await?;
	create_index(database, crate::database::COLLECTION_USERS, "id").await?;
	create_index(database, crate::database::COLLECTION_PHOTOS, "id").await?;
	create_index(database, crate::database::COLLECTION_ALBUMS, "id").await?;
	Ok(())
}

pub async fn find_one<T: serde::de::DeserializeOwned>(
	collection: &str,
	id: &str,
	limit_fields: Option<Vec<String>>,
) -> Result<Option<T>> {
	let mut items: Vec<T> = find_many(collection, None, Some(&[id]), None, limit_fields).await?;

	if !items.is_empty() {
		Ok(items.pop())
	} else {
		Ok(None)
	}
}

pub async fn find_many<T: serde::de::DeserializeOwned>(
	collection: &str,
	user_id: Option<&str>,
	ids: Option<&[&str]>,
	sort_field: Option<&SortField<'_>>,
	limit_fields: Option<Vec<String>>,
) -> Result<Vec<T>> {
	let mongo_collection = DATABASE.get().await.collection::<bson::Document>(collection);
	let mut pipeline = vec![doc! {
		"$match": create_filter_for_user_and_ids_options(&user_id, &ids)
	}];

	// Add $sort stage to pipeline
	if let Some(sort) = sort_field {
		pipeline.push(doc! {
			"$sort": {
				sort.field: if sort.ascending { 1 } else { -1 }
			}
		});
	}

	if let Some(fields) = limit_fields {
		let mut project_stage_fields = doc! {};

		for field in fields {
			project_stage_fields.insert(field, 1u32);
		}

		pipeline.push(doc! {
			"$project": project_stage_fields
		});
	}

	// Run query and collect results
	match mongo_collection.aggregate(pipeline, None).await {
		Ok(cursor) => Ok(get_items_from_cursor(cursor).await?),
		Err(error) => Err(Box::from(format!("error: {:?}", error))),
	}
}

pub async fn insert_one<'de, T: serde::Serialize + serde::Deserialize<'de>>(collection: &str, item: &T) -> Result<String> {
	let mongo_collection = DATABASE.get().await.collection::<T>(collection);
	let result = mongo_collection.insert_one(item, None).await;
	match result {
		Ok(insert_result) => match insert_result.inserted_id.as_object_id() {
			Some(object_id) => Ok(object_id.to_hex()),
			None => Err(Box::from(DatabaseError::InvalidId)),
		},
		Err(error) => Err(Box::from(error)),
	}
}

pub async fn replace_one<T: serde::Serialize>(collection: &str, id: &str, replacement: &T) -> Result<()> {
	let mongo_collection = DATABASE.get().await.collection::<T>(collection);
	let filter = create_filter_for_id(id);

	match mongo_collection.replace_one(filter, replacement, None).await {
		Ok(_) => Ok(()),
		Err(error) => Err(Box::from(error.to_string())),
	}
}

pub async fn delete_one(collection: &str, id: &str) -> Result<()> {
	let ids = vec![id];
	delete_many(collection, &ids).await
}

pub async fn delete_many(collection: &str, ids: &[&str]) -> Result<()> {
	let mongo_collection = DATABASE.get().await.collection::<bson::Document>(collection);
	let filter = create_in_filter_for_ids(ids);

	mongo_collection.delete_many(filter, None).await?;
	Ok(())
}

pub async fn find_photos(photos: Vec<FindEntity>) -> Result<Vec<upholi_lib::http::response::Photo>> {
	find_photos_with_fields::<upholi_lib::http::response::Photo>(photos, None).await
}

pub async fn find_photos_minimal(photos: Vec<FindEntity>) -> Result<Vec<PhotoMinimal>> {
	find_photos_with_fields::<PhotoMinimal>(
		photos,
		Some(doc! {
			"$project": {
				"id": "$id",
				"width": "$width",
				"height": "$height"
			}
		}),
	)
	.await
}

async fn find_photos_with_fields<T>(photos: Vec<FindEntity>, project_stage: Option<bson::Document>) -> Result<Vec<T>>
where
	T: serde::de::DeserializeOwned,
{
	let mongo_collection = DATABASE.get().await.collection::<Photo>(database::COLLECTION_PHOTOS);

	let mut photo_filter_docs: Vec<bson::Document> = Vec::new();
	for photo in photos {
		let filter_doc = match photo.key_hash {
			Some(key_hash) => {
				doc! {
					"id": photo.id,
					"keyHash": key_hash
				}
			}
			None => {
				doc! {
					"id": photo.id,
				}
			}
		};

		photo_filter_docs.push(filter_doc);
	}

	if !photo_filter_docs.is_empty() {
		let mut pipeline = vec![doc! {
			"$match": {
				"$or": photo_filter_docs
			}
		}];

		if let Some(project_stage) = project_stage {
			pipeline.push(project_stage);
		}

		let cursor = mongo_collection.aggregate(pipeline, None).await?;
		get_items_from_cursor(cursor).await
	} else {
		// Nothing to query
		Ok(vec![])
	}
}

pub async fn photo_exists_for_user(user_id: &str, hash: &str) -> Result<bool> {
	let mongo_collection = DATABASE.get().await.collection::<Photo>(database::COLLECTION_PHOTOS);
	let filter = doc! {
		"userId": user_id,
		"hash": hash
	};

	let count = mongo_collection.count_documents(filter, None).await?;
	Ok(count > 0)
}

pub async fn get_user_by_username(username: &str) -> Result<Option<User>> {
	let mongo_collection = DATABASE.get().await.collection(database::COLLECTION_USERS);
	let query = doc! {
		"username": username,
	};

	let cursor = mongo_collection.find(query, None).await?;
	let users = get_items_from_cursor(cursor).await?;

	match users.len() {
		1 => Ok(Some(users.into_iter().next().unwrap())),
		0 => Ok(None),
		_ => Err(Box::from(format!("Multiple users found with username '{}'", username))),
	}
}

pub async fn find_shares(user_id: &str, filters: FindSharesFilter) -> Result<Vec<Share>> {
	let mongo_collection = DATABASE.get().await.collection(database::COLLECTION_SHARES);
	let mut query = doc! {
		"userId": user_id,
	};

	if let Some(identifier_hash) = filters.identifier_hash {
		query.extend(doc! {"identifierHash": identifier_hash});
	}

	let cursor = mongo_collection.find(query, None).await?;
	get_items_from_cursor(cursor).await
}

/// Take all items available in given cursor. This exhausts the cursor.
async fn get_items_from_cursor<T: serde::de::DeserializeOwned>(mut cursor: mongodb::Cursor<bson::Document>) -> Result<Vec<T>> {
	let mut items = Vec::new();

	while let Some(document) = cursor.try_next().await? {
		match bson::from_bson(bson::Bson::Document(document)) {
			Ok(item) => items.push(item),
			Err(error) => {
				return Err(Box::from(error));
			}
		}
	}

	Ok(items)
}

/// Create a filter definition to be used in queries that matches an item with given id
fn create_filter_for_id(id: &str) -> bson::Document {
	doc! {"id": id}
}

/// Create a filter definition to be used in queries that matches all items with given ids
fn create_in_filter_for_ids(ids: &[&str]) -> bson::Document {
	doc! {"id": doc!{"$in": ids } }
}

/// Create a filter definition to be used in queries that matches all item for a user
fn create_filter_for_user(user_id: &str) -> bson::Document {
	doc! {"userId": user_id}
}

/// Create a filter definition to be used in queries that matches all item for a user and certian item ids
fn create_filter_for_user_and_ids(user_id: &str, ids: &[&str]) -> bson::Document {
	doc! {
		"id": doc!{"$in": ids },
		"userId": user_id
	}
}

/// Create a filter definition to be used in queries that matches all item for a user and certian item ids
fn create_filter_for_user_and_ids_options(user_id: &Option<&str>, ids: &Option<&[&str]>) -> bson::Document {
	if user_id.is_some() && ids.is_some() {
		create_filter_for_user_and_ids(user_id.unwrap(), ids.unwrap())
	} else if user_id.is_some() && ids.is_none() {
		create_filter_for_user(user_id.unwrap())
	} else if user_id.is_none() && ids.is_some() {
		create_in_filter_for_ids(ids.unwrap())
	} else {
		doc! {}
	}
}

/// Create an index if it does not exist, this index will have option 'unique: true'.
/// Note: existance check if by name only, does not take index options into account
async fn create_index(database: &mongodb::Database, collection: &str, key: &str) -> Result<()> {
	if !index_exists(database, collection, key).await? {
		let command = doc! {
			"createIndexes": collection,
			"indexes": vec!{
				doc!{
					"key": {
						key: 1
					},
					"name": key,
					"unique": true
				}
			}
		};

		database.run_command(command, None).await?;
	}

	Ok(())
}

/// Check if index with given name exists for collection
async fn index_exists(database: &mongodb::Database, collection: &str, index_name: &str) -> Result<bool> {
	let command = doc! {
		"listIndexes": collection
	};

	let result = database.run_command(command, None).await?;
	let index_docs = result.get_document("cursor")?.get_array("firstBatch")?;

	let exists = index_docs.iter().any(|bson| match bson.as_document() {
		Some(doc) => match doc.get_str("name") {
			Ok(name) => name == index_name,
			Err(_) => false,
		},
		None => false,
	});

	Ok(exists)
}
