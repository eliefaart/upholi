use mongodb::{Client, options::ClientOptions};
use bson::{doc};
use lazy_static::lazy_static;
use crate::error::*;

pub mod album;
pub mod photo;

const COLLECTION_SESSIONS: &str = "sessions";
const COLLECTION_PHOTOS: &str = "photos";
const COLLECTION_ALBUMS: &str = "albums";

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


pub fn get_collection_photos() -> mongodb::Collection {
	DATABASE.collection(COLLECTION_PHOTOS)
}

pub fn get_collection_albums() -> mongodb::Collection {
	DATABASE.collection(COLLECTION_ALBUMS)
}

pub fn get_collection_sessions() -> mongodb::Collection {
	DATABASE.collection(COLLECTION_SESSIONS)
}

pub struct SortField<'a> {
	pub field: &'a str,
	pub ascending: bool,
}

/// Add standard CRUD operations to a struct
pub trait DatabaseOperations {
	/// Get an existing item
	fn get(id: &str) -> Option<Self>
		where Self: std::marker::Sized;

	/// Insert item as new record
	fn insert(&self) -> Result<()>;

	/// Store this instance in its current state
	fn update(&self) -> Result<()>;

	/// Delete this item from database
	fn delete(&self) -> Result<()>;
}

/// Adds CRUD operations to a struct that targets multiple items
pub trait DatabaseBatchOperations {
	/// Get all items with an id contained within given array
	fn get_with_ids(ids: &[&str]) -> Result<Vec<Self>>
		where Self: std::marker::Sized;
}

/// Add database operations to a struct, which are targetted only to entries owned by given user
pub trait DatabaseUserOperations: DatabaseOperations {
	fn get_as_user(id: &str, user_id: i64) -> Result<Option<Self>>
		where Self: std::marker::Sized;

	fn get_all_as_user(user_id: i64) -> Result<Vec<Self>>
		where Self: std::marker::Sized;

	fn get_all_with_ids_as_user(ids: &[&str], user_id: i64) -> Result<Vec<Self>>
		where Self: std::marker::Sized;
}


/// Insert a single item into a collection
pub fn insert_item<T: serde::Serialize>(collection: &mongodb::Collection, item: &T) -> Result<String> {
	match bson::to_bson(item) {
		Ok(bson_item) => {
			if let bson::Bson::Document(document) = bson_item {
				let result = collection.insert_one(document, None);
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

/// Get a single item from a collection
pub fn find_one<'de, T: serde::Deserialize<'de>>(id: &str, collection: &mongodb::Collection) -> Option<T> {
	match find_many(collection, None, Some(&[id]), None) {
		Ok(mut items) => items.pop(),
		Err(_) => None
	}
}

/// Get multiple items from a collection
pub fn find_many<'de, T: serde::Deserialize<'de>>(collection: &mongodb::Collection, user_id: Option<i64>, ids: Option<&[&str]>, sort_field: Option<&SortField>) -> Result<Vec<T>> {
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
	match collection.aggregate(pipeline, None) {
		Ok(cursor) => Ok(get_items_from_cursor(cursor)?),
		Err(error) => Err(Box::from(format!("error: {:?}", error)))
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

/// Replace a single existing item with a new version in its entirety
pub fn replace_one<T: serde::Serialize>(id: &str, replacement: &T, collection: &mongodb::Collection) -> Result<()> {
	let filter = create_filter_for_id(id);
	match bson::to_bson(&replacement) {
		Ok(bson_item) => {
			if let bson::Bson::Document(document) = bson_item {
				match collection.replace_one(filter, document, None) {
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

/// Delete an item from a collection
pub fn delete_one(id: &str, collection: &mongodb::Collection) -> Option<()> {
	let ids = vec!{ id };
	delete_many(&ids, &collection)
}

/// Delete multiple items from a collection
fn delete_many(ids: &[&str], collection: &mongodb::Collection) -> Option<()> {
	let filter = create_in_filter_for_ids(ids);
	let result = collection.delete_many(filter, None);

	match result {
		Ok(delete_result) => {
			if delete_result.deleted_count > 0 {
				Some(())
			} else {
				None
			}
		},
		Err(_) => None
	}
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
