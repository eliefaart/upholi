use bson::{doc};

use crate::database;
use crate::types;
use crate::albums;

pub fn create(album: &albums::Album) -> Option<String> {
	let collection = get_collection();
	let bson_album = album.to_bson_album();

	database::insert_item(&collection, &bson_album)
}

pub fn get(id: &str) -> Option<albums::Album> {
	let collection = get_collection();
	let result: Option<types::BsonAlbum> = database::find_one(&id, &collection);
	
	match result {
		Some(bson_album) => Some(bson_album.to_album()),
		None => None
	}
}

pub fn get_all() -> Vec<albums::Album> {
	let collection = get_collection();

	let find_result = collection.find(None, None);
	let cursor = find_result.unwrap();
	
	let mut albums: Vec<albums::Album> = Vec::new();
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
	} else {
		None
	}
}

pub fn remove_photo_from_all_albums(photo_id: &str) -> Result<(), ()> {
	let ids = vec!{ photo_id };
	remove_photos_from_all_albums(&ids)
}

pub fn remove_photos_from_all_albums(photo_ids: &Vec<&str>) -> Result<(), ()> {
	let collection = get_collection();

	let mut object_ids = Vec::new();
	for photo_id in photo_ids {
		let result = types::string_to_object_id(&photo_id.to_string());
		if let Some(object_id) = result {
			object_ids.push(object_id);
		}
	}

	let query = doc!{
		"photos": doc!{
			"$in": &object_ids
		}
	};
	let update = doc!{
		"$pull": doc!{
			"photos": doc!{
				"$in": &object_ids
			}
		}
	};

	let result = collection.update_many(query, update, None);
	match result {
		Ok(_) => Ok(()),
		Err(_) => Err(())
	}
}

/// Unset thumbnail of all album where thumbnail is set to given photo_id
pub fn remove_thumb_from_all_albums(photo_id: &str) -> Result<(), ()> {
	let ids = vec!{ photo_id };
	remove_thumbs_from_all_albums(&ids)
}

/// Unset thumbnail of all album where thumbnail is set to any of given photo_ids
pub fn remove_thumbs_from_all_albums(photo_ids: &Vec<&str>) -> Result<(), ()> {
	let collection = get_collection();

	let mut object_ids = Vec::new();
	for photo_id in photo_ids {
		let result = types::string_to_object_id(&photo_id.to_string());
		if let Some(object_id) = result {
			object_ids.push(object_id);
		}
	}

	let query = doc!{
		"thumb_photo_id": doc!{
			"$in": &object_ids
		}
	};
	let update = doc!{
		"$set": doc!{
			"thumb_photo_id": bson::Bson::Null
		}
	};

	let result = collection.update_many(query, update, None);
	match result {
		Ok(_) => Ok(()),
		Err(_) => Err(())
	}
}


fn get_collection() -> mongodb::Collection {
	let db = database::get_database();
	db.collection(database::COLLECTION_ALBUMS)
}