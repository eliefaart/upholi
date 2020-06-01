use bson::{doc};

use crate::database;
use crate::types;
use crate::albums;

pub fn insert(album: &albums::Album) -> Result<String, String> {
	if album.id.is_empty() {
		return Err("Album ID not set".to_string());
	}

	let collection = get_collection();
	database::insert_item(&collection, &album)
}

pub fn get(id: &str) -> Option<albums::Album> {
	let collection = get_collection();
	let result: Option<albums::Album> = database::find_one(&id, &collection);
	
	result
}

pub fn get_all() -> Vec<albums::Album> {
	let collection = get_collection();

	let find_result = collection.find(None, None);
	let cursor = find_result.unwrap();
	
	let mut albums: Vec<albums::Album> = Vec::new();
	for result in cursor {
		match result {
			Ok(document) => {
				let album: albums::Album = bson::from_bson(bson::Bson::Document(document)).unwrap();
				albums.push(album);
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

pub fn update(id: &str, updated_album: &types::UpdateAlbum) -> Option<()> {
	let collection = get_collection();

	if let Some(mut album) = get(id) {
		
		if updated_album.title.is_some() {
			album.title = updated_album.title.as_ref().unwrap().to_string();
		}
		if updated_album.photos.is_some() {
			album.photos = updated_album.photos.as_ref().unwrap().to_vec();
		}
		if updated_album.thumb_photo_id.is_some() {
			album.thumb_photo_id = Some(updated_album.thumb_photo_id.as_ref().unwrap().to_string());
		}

		let result = database::create_filter_for_id(&album.id);

		if let Some(filter) = result {
			let serialized_bson = bson::to_bson(&album).unwrap();

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
	} else {
		None
	}
}

/// Remove photos with given photo_ids from all albums containing any of these photos
pub fn remove_photos_from_all_albums(photo_ids: &Vec<&str>) -> Result<(), ()> {
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
pub fn remove_thumbs_from_all_albums(photo_ids: &Vec<&str>) -> Result<(), ()> {
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
		Ok(res) => {
			Ok(())
		},
		Err(_) => Err(())
	}
}


fn get_collection() -> mongodb::Collection {
	database::DATABASE.collection(database::COLLECTION_ALBUMS)
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn insert_empty_id() {
		let album = create_dummy_album_with_id("");
		let result = insert(&album);

		assert!(result.is_err());
	}

	fn create_dummy_album_with_id(id: &str) -> albums::Album {
		albums::Album{
			id: id.to_string(),
			title: "title".to_string(),
			thumb_photo_id: Some(bson::oid::ObjectId::new().unwrap().to_hex()),
			photos: vec!{
				bson::oid::ObjectId::new().unwrap().to_hex(),
				bson::oid::ObjectId::new().unwrap().to_hex(),
				bson::oid::ObjectId::new().unwrap().to_hex()
			}
		}
	}
}