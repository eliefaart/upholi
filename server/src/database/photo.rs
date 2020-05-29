use bson::{doc};

use crate::database;
use crate::types;
use crate::photos;

pub fn create(photo: &photos::Photo) -> Option<String> {
	let collection = get_collection();
	let bson_photo = photo.to_bson_photo();

	database::insert_item(&collection, &bson_photo)
}

pub fn get(id: &str) -> Option<photos::Photo> {
	let collection = get_collection();
	let result: Option<types::BsonPhoto> = database::find_one(&id, &collection);

	match result {
		Some(bson_photo) => Some(bson_photo.to_photo()),
		None => None
	}
}

pub fn get_many(ids: &Vec<&str>) -> Option<Vec<photos::Photo>> {
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

pub fn get_all() -> Vec<photos::Photo> {
	let collection = get_collection();

	let mut find_options: mongodb::options::FindOptions = Default::default();
	find_options.sort = Some(doc!{ "_id": -1});

	let find_result = collection.find(None, find_options);
	let cursor = find_result.unwrap();
	
	let mut photos: Vec<photos::Photo> = Vec::new();
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

pub fn hash_exists(hash: &str) -> Result<bool, String> {
	let collection = get_collection();
	let filter = doc! { "hash": hash };
	let result = collection.count_documents(filter, None);

	match result {
		Ok(count) => Ok(count > 0),
		Err(err) => Err(format!("{:?}", err))
	}
}

fn get_collection() -> mongodb::Collection {
	let db = database::get_database();
	db.collection(database::COLLECTION_PHOTOS)
}