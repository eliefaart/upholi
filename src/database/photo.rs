use crate::database;
use crate::types;

pub fn create(photo: &types::Photo) -> Option<String> {
	let db = database::get_database();
	let collection = db.collection(database::COLLECTION_PHOTOS);

	let bson_photo = photo.to_bson_photo();
	database::insert_item(&collection, &bson_photo)
}

pub fn get_one(id: &str) -> Option<types::Photo> {
	let db = database::get_database();
	let collection = db.collection(database::COLLECTION_PHOTOS);

	let result: Option<types::BsonPhoto> = database::find_one(&id, &collection);
	match result {
		Some(bson_photo) => Some(bson_photo.to_photo()),
		None => None
	}
}

pub fn get_all() -> Vec<types::Photo> {
	let db = database::get_database();
	let collection = db.collection(database::COLLECTION_PHOTOS);

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