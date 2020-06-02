use bson::{doc};

use crate::database;
use crate::photos;

pub fn insert(photo: &photos::Photo) -> Result<String, String> {
	if photo.id.is_empty() {
		return Err("Photo ID not set".to_string());
	}

	let collection = get_collection();
	database::insert_item(&collection, &photo)
}

pub fn get(id: &str) -> Option<photos::Photo> {
	let collection = get_collection();
	let result: Option<photos::Photo> = database::find_one(&id, &collection);

	result
}

pub fn get_many(ids: &Vec<&str>) -> Option<Vec<photos::Photo>> {
	let collection = get_collection();
	let result: Option<Vec<photos::Photo>> = database::find_many(&ids, &collection);

	match result {
		Some(photos) =>	Some(photos),
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
				let photo: photos::Photo = bson::from_bson(bson::Bson::Document(document)).unwrap();
				photos.push(photo);
			}
			Err(e) => println!("Error in cursor: {:?}", e),
		}
	}

	photos
}

pub fn delete(id: &str) -> Option<()>{
	delete_many(&vec!{id})
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
	database::DATABASE.collection(database::COLLECTION_PHOTOS)
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn insert_empty_id() {
		let photo = create_dummy_photo_with_id("");
		let result = insert(&photo);

		assert!(result.is_err());
	}

	fn create_dummy_photo_with_id(id: &str) -> photos::Photo {
		photos::Photo {
			id: id.to_string(),
			name: "photo name".to_string(),
			width: 150,
			height: 2500,
			hash: "abc123".to_string(),
			path_thumbnail: "path_thumbnail".to_string(),
			path_preview: "path_preview".to_string(),
			path_original: "path_original".to_string(),
			exif: crate::exif::Exif {
				manufactorer: None,
				model: None,
				aperture: None,
				exposure_time: None,
				iso: None,
				focal_length: None,
				focal_length_35mm_equiv: None,
				orientation: None,
				date_taken: None
			}
		}
	}
}