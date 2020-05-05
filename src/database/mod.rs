use mongodb::{Client, options::ClientOptions};
use bson::{doc};

pub mod album;
pub mod photo;

const APP_NAME: &str = "Hummingbird";
const CONNECTION_STRING: &str = "mongodb://localhost:27017";	// TODO: Figure out how to do config file in rust, or another way to get secrets.
const COLLECTION_PHOTOS: &str = "photos";
const COLLECTION_ALBUMS: &str = "albums";

//static mut DATABASE: Option<mongodb::Database> = None;

fn get_database() -> mongodb::Database {
	init_database(CONNECTION_STRING).unwrap()
// 	unsafe {
// 		match &DATABASE {
// 			Some(_x) => (),
// 			None => DATABASE = Some(init_database(CONNECTION_STRING).unwrap())
// 		}

// 		DATABASE.unwrap()
// 	}
}

fn init_database(connection_string: &str) -> Result<mongodb::Database, mongodb::error::Error> {

	let mut client_options = ClientOptions::parse(connection_string)?;
	client_options.app_name = Some(APP_NAME.to_string());

	// Get a handle to the deployment.
	let client = Client::with_options(client_options)?;
	let database = client.database("rust");

	Ok(database)
}

fn insert_item<T: serde::Serialize>(collection: &mongodb::Collection, bson_item: &T) -> Option<String> {
	let serialized_bson = bson::to_bson(bson_item).unwrap();

	// I don't fully understand this syntax. 
	// Something like: if serialized_bson destructures into bson::Bson::Document document succesfully then..
	// I guess I understand the 'if let' syntax, but not the bson::Bson::Document(document) = serialized_bson part.
	if let bson::Bson::Document(document) = serialized_bson {
		let result = collection.insert_one(document, None);
		match result {
			Ok(insert_result) => Some(insert_result.inserted_id.as_object_id().unwrap().to_hex()),
			Err(_) => None
		}
	} else {
		None
	}
}

fn find_one<'de, T: serde::Deserialize<'de>>(id: &str, collection: &mongodb::Collection) -> Option<T> {
	let result = create_filter_for_id(id);
	if let Some(filter) = result {
		let find_result = collection.find_one(filter, None);

		match find_result {
			Ok(document_option) => {
				let document = document_option.unwrap();
				let item = bson::from_bson(bson::Bson::Document(document)).unwrap();
	
				Some(item)
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
	let result = create_filter_for_id(id);
	if let Some(filter) = result {
		let result = collection.delete_one(filter, None);

	match result {
		Ok(delete_result) => {
			if delete_result.deleted_count == 1 {
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
	let result = bson::oid::ObjectId::with_string(id);
	match result {
		Ok(object_id) => Some(doc!{"_id": object_id}),
		Err(_) => None
	}
}