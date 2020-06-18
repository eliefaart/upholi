use mongodb::{Client, options::ClientOptions};
use bson::{doc};
use lazy_static::lazy_static;

pub mod album;
pub mod photo;
pub mod session;

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

/// Add standard CRUD operations to a struct
pub trait DatabaseOperations {
	/// Get an existing item
	fn get(id: &str) -> Option<Self>
		where Self: std::marker::Sized;

	/// Insert item as new record
	fn insert(&self) -> Result<(), String>;

	/// Store this instance in its current state
	fn update(&self) -> Result<(), String>;

	/// Delete this item from database
	fn delete(&self) -> Result<(), String>;
}

/// Add database operations to a struct, which are targetted only to entries owned by given user
pub trait DatabaseUserOperations: DatabaseOperations {
	fn get_all(user_id: i64) -> Result<Vec<Self>, String>
		where Self: std::marker::Sized;

	fn get_all_with_ids(_user_id: i64, ids: &[&str]) -> Result<Vec<Self>, String>
		where Self: std::marker::Sized;
}


/// Insert a single item into a collection
pub fn insert_item<T: serde::Serialize>(collection: &mongodb::Collection, bson_item: &T) -> Result<String, String> {
	let serialized_bson = bson::to_bson(bson_item).unwrap();

	if let bson::Bson::Document(document) = serialized_bson {
		let result = collection.insert_one(document, None);
		match result {
			Ok(insert_result) => Ok(insert_result.inserted_id.as_object_id().unwrap().to_hex()),
			Err(err) => Err(err.to_string())
		}
	} else {
		Err("Failed to serialize struct".to_string())
	}
}

/// Get a single item from a collection
pub fn find_one<'de, T: serde::Deserialize<'de>>(id: &str, collection: &mongodb::Collection) -> Option<T> {
	match find_many_new(None, Some(&[id]), collection) {
		Ok(mut items) => items.pop(),
		Err(_) => None
	}
}

/// Get multiple items from a collection
pub fn find_many_new<'de, T: serde::Deserialize<'de>>(user_id: Option<i64>, ids: Option<&[&str]>, collection: &mongodb::Collection) -> Result<Vec<T>, String> {
	let filter = {
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
	};
	
	find_many_with_filter(filter, collection)
}

/// Get multiple items from a collection using given filter
fn find_many_with_filter<'de, T: serde::Deserialize<'de>>(filter: bson::ordered::OrderedDocument, collection: &mongodb::Collection) -> Result<Vec<T>, String> {
	let find_result = collection.find(Some(filter), None);

	match find_result {
		Ok(cursor) => {
			let mut items = Vec::new();

			for document_result in cursor {
				let document = document_result.unwrap();
				let item = bson::from_bson(bson::Bson::Document(document)).unwrap();
				items.push(item);
			}

			Ok(items)
		},
		Err(error) => Err(format!("error: {:?}", error))
	}
}

/// Replace a single existing item with a new version in its entirety
pub fn replace_one<T: serde::Serialize>(id: &str, replacement: &T, collection: &mongodb::Collection) -> Result<(), String> {
	let filter = create_filter_for_id(id);
	match bson::to_bson(&replacement) {
		Ok(serialized_bson) => {
			if let bson::Bson::Document(document) = serialized_bson {
				match collection.replace_one(filter, document, None) {
					Ok(_) => Ok(()),
					Err(error) => Err(error.to_string())
				}
			}
			else {
				Err("Invalid bson document".to_string())
			}
		},
		Err(error) => Err(error.to_string())
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