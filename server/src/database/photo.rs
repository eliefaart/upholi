use bson::{doc};

use crate::database;

pub fn delete_many(ids: &[&str]) -> Result<(), String> {
	let collection = get_collection();
	if database::album::remove_photos_from_all_albums(ids).is_ok() {
		if database::album::remove_thumbs_from_all_albums(ids).is_ok() {
			match database::delete_many(ids, &collection) {
				Some(_) => Ok(()),
				None => Err("Failed to delete photos from database".to_string())
			}
		} else {
			Err("Failed to unset cover photos from albums".to_string())
		}
	} else {
		Err("Failed to remove photos from albums".to_string())
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

pub fn get_collection() -> mongodb::Collection {
	database::DATABASE.collection(database::COLLECTION_PHOTOS)
}