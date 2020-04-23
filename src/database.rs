use mongodb::{Client, options::ClientOptions};
use bson::{doc};

const APP_NAME: &str = "Hummingbird";

pub fn get_database() -> Result<mongodb::Database, mongodb::error::Error> {

	let connection_string = "mongodb://localhost:27017";

	// Parse a connection string into an options struct.
	let mut client_options = ClientOptions::parse(connection_string)?;

	// Manually set an option.
	client_options.app_name = Some(APP_NAME.to_string());

	// Get a handle to the deployment.
	let client = Client::with_options(client_options)?;

	// Get a handle to a database.
	let database = client.database("rust");

	Ok(database)
}

pub fn insert_docs(database: mongodb::Database) {
	// Get a handle to a collection in the database.
	let collection = database.collection("books");

	let docs = vec![
		doc! { "title": "1984", "author": "George Orwell" },
		doc! { "title": "Animal Farm", "author": "George Orwell" },
		doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
	];

	// Insert some documents into the "mydb.books" collection.
	let result = collection.insert_many(docs, None);
	match result {
		Ok(_v) => (),
		Err(e) => println!("error: {:?}", e),
	}
}