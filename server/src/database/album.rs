use bson::{doc};

use crate::database;

/// Remove photos with given photo_ids from all albums containing any of these photos
pub fn remove_photos_from_all_albums(photo_ids: &[&str]) -> Result<(), ()> {
	let collection = get_collection();

	let query = doc!{
		"photos": doc!{
			"$in": &photo_ids
		}
	};
	let update = doc!{
		"$pull": doc!{
			"photos": doc!{
				"$in": &photo_ids
			}
		}
	};

	let result = collection.update_many(query, update, None);
	match result {
		Ok(_) => Ok(()),
		Err(_) => Err(())
	}
}

/// Unset thumbnail of all album where thumbnail is set to any of given photo_ids
pub fn remove_thumbs_from_all_albums(photo_ids: &[&str]) -> Result<(), ()> {
	let collection = get_collection();

	let query = doc!{
		"thumbPhotoId": doc!{
			"$in": &photo_ids
		}
	};
	let update = doc!{
		"$set": doc!{
			"thumbPhotoId": bson::Bson::Null
		}
	};

	let result = collection.update_many(query, update, None);
	match result {
		Ok(_) => Ok(()),
		Err(_) => Err(())
	}
}


pub fn get_collection() -> mongodb::Collection {
	database::DATABASE.collection(database::COLLECTION_ALBUMS)
}