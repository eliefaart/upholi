use serde::{Serialize, Deserialize};

use crate::ids;
use crate::database;
use crate::database::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	#[serde(default)] 
	pub id: String,
	pub user_id: i64,
	pub title: String,
	#[serde(default)]
	pub thumb_photo_id: Option<String>,
	#[serde(default)]
	pub photos: Vec<String>
}

impl Album {
	pub fn new(user_id: i64, title: &str) -> Self {
		let id = ids::create_unique_id();

		Self {
			id,
			user_id,
			title: title.to_string(),
			thumb_photo_id: None,
			photos: vec!{}
		}
	}
}

impl DatabaseOperations for Album {
	fn get(id: &str) -> Option<Self> {
		let collection = database::get_collection_albums();
		database::find_one(&id, &collection)
	}

	fn insert(&self) -> Result<(), String> {
		if self.id.is_empty() {
			return Err("Album ID not set".to_string());
		}

		let collection = database::get_collection_albums();
		
		match Self::get(&self.id) {
			Some(_) => Err(format!("An album with id {} already exists", &self.id)),
			None => {
				let _ = database::insert_item(&collection, &self)?;
				Ok(())
			}
		}
	}

	fn update(&self) -> Result<(), String> {
		let collection = database::get_collection_albums();
		database::replace_one(&self.id, self, &collection)
	}

	fn delete(&self) -> Result<(), String> {
		let collection = database::get_collection_albums();
		match database::delete_one(&self.id, &collection) {
			Some(_) => Ok(()),
			None => Err("Failed to delete album".to_string())
		}
	}
}

impl DatabaseUserOperations for Album {
	fn get_as_user(id: &str, user_id: i64) -> Result<Option<Self>, String>{
		match Self::get(id) {
			Some(album) => {
				if album.user_id != user_id {
					Err(format!("User {} does not have access to album {}", user_id, album.id))
				} else {
					Ok(Some(album))
				}
			}
			None => Ok(None)
		}
	}

	fn get_all_as_user(user_id: i64) -> Result<Vec<Self>, String> {
		let collection = database::get_collection_albums();
		database::find_many(Some(user_id), None, &collection) 
	}

	fn get_all_with_ids_as_user(ids: &[&str], user_id: i64) -> Result<Vec<Self>, String> {
		let collection = database::get_collection_albums();
		database::find_many(Some(user_id), Some(ids), &collection) 
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		const TITLE: &str = "Hello world";
		const USER_ID: i64 = 100i64;

		let album = Album::new(USER_ID, TITLE);

		assert!(!album.id.is_empty());
		assert_eq!(album.title, TITLE);
		assert_eq!(album.user_id, USER_ID);
	}

	#[test]
	fn insert_empty_id() {
		let album = create_dummy_album_with_id("");
		let result = album.insert();

		assert!(result.is_err());
	}

	fn create_dummy_album_with_id(id: &str) -> Album {
		Album{
			id: id.to_string(),
			user_id: 0i64,
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