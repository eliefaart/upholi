use crate::database;
use crate::types;

pub fn create(photo: &types::Photo) -> Option<String> {
	let collection = get_collection();
	let bson_photo = photo.to_bson_photo();

	database::insert_item(&collection, &bson_photo)
}

pub fn get(id: &str) -> Option<types::Photo> {
	let collection = get_collection();
	let result: Option<types::BsonPhoto> = database::find_one(&id, &collection);

	match result {
		Some(bson_photo) => Some(bson_photo.to_photo()),
		None => None
	}
}

pub fn get_many(ids: &Vec<&str>) -> Option<Vec<types::Photo>> {
	let collection = get_collection();
	let result: Option<Vec<types::BsonPhoto>> = database::find_many(&ids, &collection);

	match result {
		Some(bson_photos) => {
			let mut photos = Vec::new();
			for bson_photo in bson_photos {
				photos.push(bson_photo.to_photo());
			}
			Some(photos)
		},
		None => None
	}
}

pub fn get_all() -> Vec<types::Photo> {
	let collection = get_collection();

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

pub fn delete(id: &str) -> Option<()>{
	let collection = get_collection();
	if let Ok(_) = database::album::remove_photo_from_all_albums(id) {
		if let Ok(_) = database::album::remove_thumb_from_all_albums(id) {
			database::delete_one(id, &collection)
		} else {
			None
		}
	} else {
		None
	}
}

pub fn delete_many(ids: &Vec<&str>) -> Option<()>{
	let collection = get_collection();
	if let Ok(_) = database::album::remove_photos_from_all_albums(ids) {
		if let Ok(_) = database::album::remove_thumbs_from_all_albums(ids) {
			database::delete_many(ids, &collection)
		} else {
			None
		}
	} else {
		None
	}
}

fn get_collection() -> mongodb::Collection {
	let db = database::get_database();
	db.collection(database::COLLECTION_PHOTOS)
}