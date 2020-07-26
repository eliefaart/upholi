
use mongodb::{Client, options::ClientOptions};
use bson::doc;
use lazy_static::lazy_static;
use crate::database;
use crate::database::{Database, DatabaseExt, SortField};
use crate::error::*;

lazy_static!{
	/// A reference to the database that can be used to execute queries etc
	static ref DATABASE: mongodb::Database = {
		let client_options = ClientOptions::parse(&crate::SETTINGS.database.connection_string)
			.expect("Failed to parse database connection string");

		let client = Client::with_options(client_options)
			.expect("Failed to initialize database client");

		client.database(&crate::SETTINGS.database.name)
	};
}

pub struct MongoDatabase {}

impl Database for MongoDatabase {
	fn find_one<'de, T: serde::Deserialize<'de>>(&self, collection: &str, id: &str) 
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

	fn find_many<'de, T: serde::Deserialize<'de>>(&self, collection: &str, user_id: Option<i64>, ids: Option<&[&str]>, sort_field: Option<&SortField>) 
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
	fn remove_photos_from_all_albums(&self, photo_ids: &[&str]) -> Result<()>
	{
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

	fn remove_thumbs_from_all_albums(&self, photo_ids: &[&str]) -> Result<()>
	{
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

	fn photo_exists_for_user(&self, user_id: i64, hash: &str) -> Result<bool>
	{
		let mongo_collection = DATABASE.collection(database::COLLECTION_PHOTOS);
		let filter = doc!{ 
			"user_id": user_id,
			"hash": hash 
		};

		let count = mongo_collection.count_documents(filter, None)?;
		Ok(count > 0)
	}
}

/// Take all items available in given cursor
fn get_items_from_cursor<'de, T: serde::Deserialize<'de>>(cursor: mongodb::Cursor) -> Result<Vec<T>> {
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
fn create_filter_for_id(id: &str) -> bson::ordered::OrderedDocument {
	doc!{"id": id}
}

/// Create a filter definition to be used in queries that matches all items with given ids
fn create_in_filter_for_ids(ids: &[&str]) -> bson::ordered::OrderedDocument {
	doc!{"id": doc!{"$in": ids } }
}

/// Create a filter definition to be used in queries that matches all item for a user
pub fn create_filter_for_user(user_id: i64) -> bson::ordered::OrderedDocument {
	doc!{"userId": user_id}
}

/// Create a filter definition to be used in queries that matches all item for a user and certian item ids
pub fn create_filter_for_user_and_ids(user_id: i64, ids: &[&str]) -> bson::ordered::OrderedDocument {
	doc!{
		"id": doc!{"$in": ids },
		"userId": user_id
	}
}

/// Create a filter definition to be used in queries that matches all item for a user and certian item ids
fn create_filter_for_user_and_ids_options(user_id: &Option<i64>, ids: &Option<&[&str]>) -> bson::ordered::OrderedDocument {
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