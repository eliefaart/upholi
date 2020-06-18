use bson::{doc};

use crate::database;

pub fn delete_many(ids: &[&str]) -> Result<(), String> {
	let collection = database::get_collection_photos();
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

pub fn exists_for_user(user_id: i64, hash: &str) -> Result<bool, String> {
	let collection = database::get_collection_photos();
	let filter = doc!{ 
		"user_id": user_id,
		"hash": hash 
	};

	match collection.count_documents(filter, None) {
		Ok(count) => Ok(count > 0),
		Err(err) => Err(format!("{:?}", err))
	}
}
