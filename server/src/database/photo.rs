use bson::{doc};
use crate::error::*;

use crate::database;

pub fn delete_many(ids: &[&str]) -> Result<()> {
	let collection = database::get_collection_photos();
	if database::album::remove_photos_from_all_albums(ids).is_ok() {
		if database::album::remove_thumbs_from_all_albums(ids).is_ok() {
			match database::delete_many(ids, &collection) {
				Some(_) => Ok(()),
				None => Err(Box::from(EntityError::DeleteFailed))
			}
		} else {
			Err(Box::from(EntityError::UpdateFailed))
		}
	} else {
		Err(Box::from(EntityError::UpdateFailed))
	}
}

pub fn exists_for_user(user_id: i64, hash: &str) -> Result<bool> {
	let collection = database::get_collection_photos();
	let filter = doc!{ 
		"user_id": user_id,
		"hash": hash 
	};

	match collection.count_documents(filter, None) {
		Ok(count) => Ok(count > 0),
		Err(err) => Err(Box::from(format!("{:?}", err)))
	}
}
