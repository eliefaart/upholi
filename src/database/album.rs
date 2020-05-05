use crate::database;
use crate::types;

pub fn create(album: &types::Album) -> Option<String> {
	let collection = get_collection();
	let bson_album = album.to_bson_album();

	database::insert_item(&collection, &bson_album)
}

pub fn get_one(id: &str) -> Option<types::Album> {
	let collection = get_collection();
	let result: Option<types::BsonAlbum> = database::find_one(&id, &collection);
	
	match result {
		Some(bson_album) => Some(bson_album.to_album()),
		None => None
	}
}

pub fn get_all() -> Vec<types::Album> {
	let collection = get_collection();

	let find_result = collection.find(None, None);
	let cursor = find_result.unwrap();
	
	let mut albums: Vec<types::Album> = Vec::new();
	for result in cursor {
		match result {
			Ok(document) => {
				let bson_album: types::BsonAlbum = bson::from_bson(bson::Bson::Document(document)).unwrap();
				albums.push(bson_album.to_album());
			}
			Err(e) => println!("Error in cursor: {:?}", e),
		}
	}

	albums
}

pub fn delete_one(id: &str) -> Option<()>{
	let collection = get_collection();
	database::delete_one(id, &collection)
}

fn get_collection() -> mongodb::Collection {
	let db = database::get_database();
	db.collection(database::COLLECTION_ALBUMS)
}