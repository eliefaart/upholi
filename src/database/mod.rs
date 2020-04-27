use mongodb::{Client, options::ClientOptions};
use bson::{doc, oid};
use serde::{Serialize};

const APP_NAME: &str = "Hummingbird";
const CONNECTION_STRING: &str = "mongodb://localhost:27017";
const COLLECTION_PHOTOS: &str = "photos";
//const COLLECTION_ALBUMS: &str = "albums";

//static mut DATABASE: Option<mongodb::Database> = None;

#[derive(Serialize)]
pub struct Photo {
	pub name: String,
	pub width: u32,
	pub height: u32,
	pub path_thumbnail: String,
	pub path_preview: String,
	pub path_original: String
}

pub fn add_photo(photo: Photo) -> Result<String, mongodb::error::Error> {
	let db = get_database();
	let collection = db.collection(COLLECTION_PHOTOS);

	let doc = doc!{
		// TODO: Utilize _id field
		
		"name": photo.name,
		"width": photo.width as u32,
		"height": photo.height as u32,
		"path_thumbnail": photo.path_thumbnail,
		"path_preview": photo.path_preview,
		"path_original": photo.path_original
	};

	// https://github.com/mongodb/bson-rust

	let result = collection.insert_one(doc, None);
	match result {
		Ok(insert_result) => Ok(insert_result.inserted_id.as_object_id().unwrap().to_hex()),
		Err(e) => Err(e)
	}
}

pub fn get_photo(photo_id: &str) -> Option<Photo> {
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
			let photo = Photo{
				name: document.get_str("name").unwrap().to_string(),
				width: document.get_i32("width").unwrap() as u32,
				height: document.get_i32("height").unwrap() as u32,
				path_thumbnail: document.get_str("path_thumbnail").unwrap().to_string(),
				path_preview: document.get_str("path_preview").unwrap().to_string(),
				path_original: document.get_str("path_original").unwrap().to_string(),
			};

			Some(photo)
		},
		Err(e) => {
			println!("error: {:?}", e);
			None
		}
	}
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