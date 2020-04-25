use mongodb::{Client, options::ClientOptions};
use bson::{doc};
use serde::{Serialize};

const APP_NAME: &str = "Hummingbird";
const CONNECTION_STRING: &str = "mongodb://localhost:27017";
const COLLECTION_PHOTOS: &str = "photos";
//const COLLECTION_ALBUMS: &str = "albums";

//static mut DATABASE: Option<mongodb::Database> = None;

#[derive(Serialize)]
pub struct Photo {
	pub id: u32,
	pub name: String,
	pub width: u16,
	pub height: u16,
	pub landscape: bool,
	pub date_taken: u32,
	pub path_thumbnail: String,
	pub path_preview: String,
	pub path_original: String
}

pub fn add_photo(photo: Photo) {
	let db = get_database();
	let collection = db.collection(COLLECTION_PHOTOS);

	let doc = doc!{
		// TODO: Utilize _id field
		"id": photo.id,
		"name": photo.name,
		"width": photo.width as u32,
		"height": photo.height as u32,
		"landscape": photo.landscape,
		"date_taken": photo.date_taken,
		"path_thumbnail": photo.path_thumbnail,
		"path_preview": photo.path_preview,
		"path_original": photo.path_original
	};

	let result = collection.insert_one(doc, None);
	match result {
		Ok(_v) => (),
		Err(e) => println!("error: {:?}", e),
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

// fn insert_docs(database: mongodb::Database) {
// 	// Get a handle to a collection in the database.
// 	let collection = database.collection("books");

// 	let docs = vec![
// 		doc! { "title": "1984", "author": "George Orwell" },
// 		doc! { "title": "Animal Farm", "author": "George Orwell" },
// 		doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
// 	];

// 	// Insert some documents into the "mydb.books" collection.
// 	let result = collection.insert_many(docs, None);
// 	match result {
// 		Ok(_v) => (),
// 		Err(e) => println!("error: {:?}", e),
// 	}
// }