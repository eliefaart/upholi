
use mongodb::{sync::Client, options::ClientOptions};
use bson::doc;
use lazy_static::lazy_static;
use upholi_lib::http::response::PhotoMinimal;
use crate::database;
use crate::database::{Database, DatabaseExt, SortField};
use crate::error::*;
use crate::entities::album::Album;
use crate::entities::collection::Collection;
use crate::entities::user::User;

lazy_static!{
	/// A reference to the database that can be used to execute queries etc
	static ref DATABASE: mongodb::sync::Database = {
		let client_options = ClientOptions::parse(&crate::SETTINGS.database.connection_string)
			.expect("Failed to parse database connection string");

		let client = Client::with_options(client_options)
			.expect("Failed to initialize database client");

		let database = client.database(&crate::SETTINGS.database.name);

		if let Err(error) = initialize(&database) {
			println!("Error preparing up database: {:?}", error);
		}

		database
	};
}

/// Initialize database by setting some indexes if needed
fn initialize(database: &mongodb::sync::Database) -> Result<()> {
	create_index(database, crate::database::COLLECTION_SESSIONS, "id")?;
	create_index(database, crate::database::COLLECTION_USERS, "id")?;
	create_index(database, crate::database::COLLECTION_PHOTOS, "id")?;
	create_index(database, crate::database::COLLECTION_ALBUMS, "id")?;
	Ok(())
}

pub struct MongoDatabase {}

impl MongoDatabase {
	pub fn new() -> Self {
		MongoDatabase{}
	}
}

impl Database for MongoDatabase {
	fn find_one<T: serde::de::DeserializeOwned>(&self, collection: &str, id: &str)
		-> Result<Option<T>>
	{
		let mut items: Vec<T> = Self::find_many(self, collection, None, Some(&[id]), None)?;

		if !items.is_empty() {
			Ok(items.pop())
		}
		else {
			Ok(None)
		}
	}

	fn find_many<T: serde::de::DeserializeOwned>(&self, collection: &str, user_id: Option<&str>, ids: Option<&[&str]>, sort_field: Option<&SortField>)
		-> Result<Vec<T>>
	{
		let mongo_collection = DATABASE.collection(collection);
		let mut pipeline = vec!{
			doc!{
				"$match": create_filter_for_user_and_ids_options(&user_id, &ids)
			}
		};

		// Add $sort stage to pipeline
		if let Some(sort) = sort_field {
			pipeline.push(doc!{
				"$sort": {
					sort.field: if sort.ascending { 1 } else { -1 }
				}
			});
		}

		// Since id is unique, we can optimize the query a bit by adding a $limit stage
		if let Some(ids_info) = ids {
			pipeline.push(doc!{
				"$limit": ids_info.len() as u32
			});
		}

		// Run query and collect results
		match mongo_collection.aggregate(pipeline, None) {
			Ok(cursor) => Ok(get_items_from_cursor(cursor)?),
			Err(error) => Err(Box::from(format!("error: {:?}", error)))
		}
	}

	fn insert_one<T: serde::Serialize>(&self, collection: &str, item: &T) -> Result<String>
	{
		let mongo_collection = DATABASE.collection(collection);

		match bson::to_bson(item) {
			Ok(bson_item) => {
				if let bson::Bson::Document(document) = bson_item {
					let result = mongo_collection.insert_one(document, None);
					match result {
						Ok(insert_result) => {
							match insert_result.inserted_id.as_object_id() {
								Some(object_id) => Ok(object_id.to_hex()),
								None => Err(Box::from(DatabaseError::InvalidId))
							}
						},
						Err(error) => Err(Box::from(error))
					}
				} else {
					Err(Box::from("Invalid bson document"))
				}
			},
			Err(error) => Err(Box::from(error))
		}
	}

	fn replace_one<T: serde::Serialize>(&self, collection: &str, id: &str, replacement: &T) -> Result<()>
	{
		let mongo_collection = DATABASE.collection(collection);
		let filter = create_filter_for_id(id);

		match bson::to_bson(&replacement) {
			Ok(bson_item) => {
				if let bson::Bson::Document(document) = bson_item {
					match mongo_collection.replace_one(filter, document, None) {
						Ok(_) => Ok(()),
						Err(error) => Err(Box::from(error.to_string()))
					}
				}
				else {
					Err(Box::from("Invalid bson document"))
				}
			},
			Err(error) => Err(Box::from(error.to_string()))
		}
	}

	fn delete_one(&self, collection: &str, id: &str) -> Result<()>
	{
		let ids = vec!{ id };
		Self::delete_many(self, &collection, &ids)
	}

	fn delete_many(&self, collection: &str, ids: &[&str]) -> Result<()> {
		let mongo_collection = DATABASE.collection(collection);
		let filter = create_in_filter_for_ids(ids);

		mongo_collection.delete_many(filter, None)?;
		Ok(())
	}
}

impl DatabaseExt for MongoDatabase {
	fn get_photos_for_user(&self, user_id: &str) -> Result<Vec<PhotoMinimal>> {
		let mongo_collection = DATABASE.collection(database::COLLECTION_PHOTOS_NEW);
		let pipeline = vec!{
			doc!{
				"$match": {
					"userId": user_id
				}
			},
			doc!{
				"$project": {
					"id": "$id",
					"width": "$width",
					"height": "$height"
				}
			}
		};

		let cursor = mongo_collection.aggregate(pipeline, None)?;
		get_items_from_cursor(cursor)
	}

	fn remove_photos_from_all_albums(&self, photo_ids: &[&str]) -> Result<()> {
		let mongo_collection = DATABASE.collection(database::COLLECTION_ALBUMS);

		let query = doc!{
			"photos": doc!{
				"$in": &photo_ids
			}
		};
		let update = doc!{
			"$pull": doc!{
				"photos": doc!{
					"$in": &photo_ids
				}
			}
		};

		mongo_collection.update_many(query, update, None)?;
		Ok(())
	}

	fn remove_thumbs_from_all_albums(&self, photo_ids: &[&str]) -> Result<()> {
		let mongo_collection = DATABASE.collection(database::COLLECTION_ALBUMS);

		let query = doc!{
			"thumbPhotoId": doc!{
				"$in": &photo_ids
			}
		};
		let update = doc!{
			"$set": doc!{
				"thumbPhotoId": bson::Bson::Null
			}
		};

		mongo_collection.update_many(query, update, None)?;
		Ok(())
	}

	fn photo_exists_for_user(&self, user_id: &str, hash: &str) -> Result<bool> {
		let mongo_collection = DATABASE.collection(database::COLLECTION_PHOTOS);
		let filter = doc!{
			"userId": user_id,
			"hash": hash
		};

		let count = mongo_collection.count_documents(filter, None)?;
		Ok(count > 0)
	}

	fn get_albums_with_photo(&self, photo_id: &str) -> Result<Vec<Album>> {
		let mongo_collection = DATABASE.collection(database::COLLECTION_ALBUMS);
		let query = doc!{
			"photos": photo_id
		};

		let cursor = mongo_collection.find(query, None)?;
		get_items_from_cursor(cursor)
	}

	fn get_user_for_identity_provider(&self, identity_provider: &str, identity_provider_user_id: &str) -> Result<Option<User>> {
		let mongo_collection = DATABASE.collection(database::COLLECTION_USERS);
		let query = doc!{
			"identityProvider": identity_provider,
			"identityProviderUserId": identity_provider_user_id
		};

		let cursor = mongo_collection.find(query, None)?;
		let users = get_items_from_cursor(cursor)?;

		match users.len() {
			1 => Ok(Some(users.into_iter().next().unwrap())),
			0 => Ok(None),
			_ => Err(Box::from(format!("Multiple users found for identity provider '{}' and identity provider user ID '{}'. There cannot be more than one.", identity_provider, identity_provider_user_id)))
		}
	}

	fn get_collection_by_share_token(&self, token: &str) -> Result<Option<Collection>> {
		let mongo_collection = DATABASE.collection(database::COLLECTION_COLLECTIONS);
		let query = doc!{
			"sharing.token": token
		};

		match mongo_collection.find_one(query, None)? {
			Some(document) => {
				let collection: Collection = bson::from_bson(bson::Bson::Document(document))?;
				Ok(Some(collection))
			},
			None => Ok(None)
		}
	}

	fn get_collections_with_album(&self, album_id: &str) -> Result<Vec<Collection>> {
		let mongo_collection = DATABASE.collection(database::COLLECTION_COLLECTIONS);
		let query = doc!{
			"albums": album_id
		};

		let cursor = mongo_collection.find(query, None)?;
		let collections = get_items_from_cursor(cursor)?;

		Ok(collections)
	}
}

/// Take all items available in given cursor. This exhausts the cursor.
fn get_items_from_cursor<T: serde::de::DeserializeOwned>(cursor: mongodb::sync::Cursor) -> Result<Vec<T>> {
	let mut items = Vec::new();

	for document_result in cursor {
		if let Ok(document) = document_result {
			match bson::from_bson(bson::Bson::Document(document)) {
				Ok(item) => items.push(item),
				Err(error) => {
					return Err(Box::from(error));
				}
			}
		}
		else {
			return Err(Box::from(DatabaseError::ReadCursorFailed));
		}
	}

	Ok(items)
}

/// Create a filter definition to be used in queries that matches an item with given id
fn create_filter_for_id(id: &str) -> bson::Document {
	doc!{"id": id}
}

/// Create a filter definition to be used in queries that matches all items with given ids
fn create_in_filter_for_ids(ids: &[&str]) -> bson::Document {
	doc!{"id": doc!{"$in": ids } }
}

/// Create a filter definition to be used in queries that matches all item for a user
fn create_filter_for_user(user_id: &str) -> bson::Document {
	doc!{"userId": user_id}
}

/// Create a filter definition to be used in queries that matches all item for a user and certian item ids
fn create_filter_for_user_and_ids(user_id: &str, ids: &[&str]) -> bson::Document {
	doc!{
		"id": doc!{"$in": ids },
		"userId": user_id
	}
}

/// Create a filter definition to be used in queries that matches all item for a user and certian item ids
fn create_filter_for_user_and_ids_options(user_id: &Option<&str>, ids: &Option<&[&str]>) -> bson::Document {
	if user_id.is_some() && ids.is_some() {
		create_filter_for_user_and_ids(user_id.unwrap(), ids.unwrap())
	}
	else if user_id.is_some() && ids.is_none() {
		create_filter_for_user(user_id.unwrap())
	}
	else if user_id.is_none() && ids.is_some() {
		create_in_filter_for_ids(ids.unwrap())
	}
	else {
		doc!{}
	}
}

/// Create an index if it does not exist, this index will have option 'unique: true'.
/// Note: existance check if by name only, does not take index options into account
fn create_index(database: &mongodb::sync::Database, collection: &str, key: &str) -> Result<()>{
	if !index_exists(database, collection, key)? {
		let command = doc!{
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

		database.run_command(command, None)?;
	}

	Ok(())
}

/// Check if index with given name exists for collection
fn index_exists(database: &mongodb::sync::Database, collection: &str, index_name: &str) -> Result<bool> {
	let command = doc!{
		"listIndexes": collection
	};

	let result = database.run_command(command, None)?;
	let index_docs = result.get_document("cursor")?.get_array("firstBatch")?;

	let exists = index_docs.iter().any(|bson|
		match bson.as_document() {
			Some(doc) => {
				match doc.get_str("name") {
					Ok(name) => name == index_name,
					Err(_) => false
				}
			},
			None => false
		});

	Ok(exists)
}