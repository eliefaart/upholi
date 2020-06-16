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

	/// Create a new item
	fn create() -> Result<Self, String>
		where Self: std::marker::Sized;

	/// Store this instance in its current state
	fn update(&self) -> Result<(), String>;

	/// Delete this item from database
	fn delete(&self) -> Result<(), String>;
}

/// Insert a single item into a collection
fn insert_item<T: serde::Serialize>(collection: &mongodb::Collection, bson_item: &T) -> Result<String, String> {
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
fn find_one<'de, T: serde::Deserialize<'de>>(id: &str, collection: &mongodb::Collection) -> Option<T> {
	let result = find_many(&[id], collection);

	if let Some(mut photos) = result {
		photos.pop()
	} else {
		None
	}
}

/// Get multiple items from a collection
fn find_many<'de, T: serde::Deserialize<'de>>(ids: &[&str], collection: &mongodb::Collection) -> Option<Vec<T>> {
	let result = create_in_filter_for_ids(ids);
	if let Some(filter) = result {
		let find_result = collection.find(filter, None);

		match find_result {
			Ok(cursor) => {
				let mut items = Vec::new();

				for document_result in cursor {
					let document = document_result.unwrap();
					let item = bson::from_bson(bson::Bson::Document(document)).unwrap();
					items.push(item);
				}
	
				Some(items)
			},
			Err(e) => {
				println!("error: {:?}", e);
				None
			}
		}
	} else {
		None
	}
}

/// Replace a single existing item with a new version in its entirety
fn replace_one<T: serde::Serialize>(id: &str, replacement: &T, collection: &mongodb::Collection) -> Result<(), String> {
	match create_filter_for_id(id) {
		Some(filter) => {
			match bson::to_bson(&replacement) {
				Ok(serialized_bson) => {
					if let bson::Bson::Document(document) = serialized_bson {
						match collection.replace_one(filter, document, None) {
							Ok(_) => Ok(()),
							Err(error) => Err(format!("{}", error))
						}
					}
					else {
						Err("Invalid bson document".to_string())
					}
				},
				Err(error) => Err(format!("{}", error))
			}
		},
		None => Err("Failed to create filter for id".to_string())
	}
}

/// Delete an item from a collection
fn delete_one(id: &str, collection: &mongodb::Collection) -> Option<()> {
	let ids = vec!{ id };
	delete_many(&ids, &collection)
}

/// Delete multiple items from a collection
fn delete_many(ids: &[&str], collection: &mongodb::Collection) -> Option<()> {
	let result = create_in_filter_for_ids(ids);
	if let Some(filter) = result {
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
	} else {
		None
	}
}

/// Create a filter definition to be used in queries that matches an item with given id
fn create_filter_for_id(id: &str) -> Option<bson::ordered::OrderedDocument> {
	Some(doc!{"id": id})
}

/// Create a filter definition to be used in queries that matches all items with given ids
fn create_in_filter_for_ids(ids: &[&str]) -> Option<bson::ordered::OrderedDocument> {
	Some(doc!{"id": doc!{"$in": ids } })
}