use mongodb::{Client, options::ClientOptions};
use bson::{doc};
use lazy_static::lazy_static;

pub mod album;
pub mod photo;

const COLLECTION_PHOTOS: &str = "photos";
const COLLECTION_ALBUMS: &str = "albums";

lazy_static!{ 
	static ref DATABASE: mongodb::Database = {
		let client_options = ClientOptions::parse(&crate::SETTINGS.database.connection_string)
			.expect("Failed to parse database connection string");

		let client = Client::with_options(client_options)
			.expect("Failed to initialize database client");

		client.database(&crate::SETTINGS.database.name)
	};
}

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

fn find_one<'de, T: serde::Deserialize<'de>>(id: &str, collection: &mongodb::Collection) -> Option<T> {
	let result = find_many(&vec!{id}, collection);

	if let Some(mut photos) = result {
		photos.pop()
	} else {
		None
	}
}

fn find_many<'de, T: serde::Deserialize<'de>>(ids: &Vec<&str>, collection: &mongodb::Collection) -> Option<Vec<T>> {
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

fn delete_one(id: &str, collection: &mongodb::Collection) -> Option<()> {
	let ids = vec!{ id };
	delete_many(&ids, &collection)
}

fn delete_many(ids: &Vec<&str>, collection: &mongodb::Collection) -> Option<()> {
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

fn create_filter_for_id(id: &str) -> Option<bson::ordered::OrderedDocument> {
	Some(doc!{"id": id})
}

fn create_in_filter_for_ids(ids: &Vec<&str>) -> Option<bson::ordered::OrderedDocument> {
	Some(doc!{"id": doc!{"$in": ids } })
}