use crate::database;
use crate::types;

pub fn create(album: &types::Album) -> Option<String> {
	let collection = get_collection();
	let bson_album = album.to_bson_album();

	database::insert_item(&collection, &bson_album)
}

pub fn get(id: &str) -> Option<types::Album> {
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

pub fn delete(id: &str) -> Option<()>{
	let collection = get_collection();
	database::delete_one(id, &collection)
}

pub fn update(album: &types::Album) -> Option<()> {
	let collection = get_collection();
	let bson_album = album.to_bson_album();
	let result = database::create_filter_for_id(&album.id);

	if let Some(filter) = result {
		let serialized_bson = bson::to_bson(&bson_album).unwrap();

		if let bson::Bson::Document(document) = serialized_bson {
			let result = collection.replace_one(filter, document, None);

			match result {
				Ok(update_result) => {
					if update_result.modified_count == 1 {
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
	} else {
		None
	}
}

fn get_collection() -> mongodb::Collection {
	let db = database::get_database();
	db.collection(database::COLLECTION_ALBUMS)
}