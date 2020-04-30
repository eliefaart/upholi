use mongodb::{Client, options::ClientOptions};
use bson::{doc};

use crate::types;

const APP_NAME: &str = "Hummingbird";
const CONNECTION_STRING: &str = "mongodb://localhost:27017";
const COLLECTION_PHOTOS: &str = "photos";
//const COLLECTION_ALBUMS: &str = "albums";

//static mut DATABASE: Option<mongodb::Database> = None;

pub fn add_photo(photo: types::Photo) -> Result<String, String> {
	let db = get_database();
	let collection = db.collection(COLLECTION_PHOTOS);

	// Photo struct to a bson document.
	// TODO: Make a generic function to convert any Struct into a bson document.
	let bson_photo = photo.to_bson_photo();
	let serialized_bson = bson::to_bson(&bson_photo).unwrap();

	// I don't fully understand this syntax. 
	// Something like: if serialized_bson destructures into bson::Bson::Document document succesfully then..
	// I guess I understand the 'if let' syntax, but not the bson::Bson::Document(document) = serialized_bson part.
	if let bson::Bson::Document(document) = serialized_bson {
		let result = collection.insert_one(document, None);
		match result {
			Ok(insert_result) => Ok(insert_result.inserted_id.as_object_id().unwrap().to_hex()),
			Err(e) => Err(e.to_string())
		}
	} else {
		Err("Error converting photo to bson document".to_string())
	}
}

pub fn get_photo(photo_id: &str) -> Option<types::Photo> {
	let db = get_database();
	let collection = db.collection(COLLECTION_PHOTOS);

	let object_id = bson::oid::ObjectId::with_string(photo_id).unwrap();

	let filter = doc!{
		"_id": object_id
	};

	let find_result = collection.find_one(filter, None);
	match find_result {
		Ok(document_option) => {
			let document = document_option.unwrap();
			let bson_photo: types::BsonPhoto = bson::from_bson(bson::Bson::Document(document)).unwrap();
			let photo = bson_photo.to_photo();

			Some(photo)
		},
		Err(e) => {
			println!("error: {:?}", e);
			None
		}
	}
}

pub fn get_photos() -> Vec<types::Photo> {
	let db = get_database();
	let collection = db.collection(COLLECTION_PHOTOS);

	let find_result = collection.find(None, None);
	let cursor = find_result.unwrap();
	
	let mut photos: Vec<types::Photo> = Vec::new();
	for result in cursor {
		match result {
			Ok(document) => {
				let bson_photo: types::BsonPhoto = bson::from_bson(bson::Bson::Document(document)).unwrap();
				photos.push(bson_photo.to_photo());
			}
			Err(e) => println!("Error in cursor: {:?}", e),
		}
	}

	photos
}

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