use serde::{Serialize, Deserialize};

use crate::ids;
use crate::database;
use crate::database::*;
use crate::error::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	#[serde(default)] 
	pub id: String,
	pub user_id: i64,
	pub title: String,
	pub public: bool,
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
			public: false,
			title: title.to_string(),
			thumb_photo_id: None,
			photos: vec!{}
		}
	}
}

impl DatabaseEntity for Album {
	fn get(id: &str) -> Option<Self> {
		match database::get_database().find_one(database::COLLECTION_ALBUMS, id) {
			Ok(item) => item,
			Err(_) => None
		}
	}

	fn insert(&self) -> Result<()> {
		if self.id.is_empty() {
			return Err(Box::from(EntityError::IdMissing));
		}

		match Self::get(&self.id) {
			Some(_) => Err(Box::from(EntityError::AlreadyExists)),
			None => {
				database::get_database().insert_one(database::COLLECTION_ALBUMS, &self)?;
				Ok(())
			}
		}
	}

	fn update(&self) -> Result<()> {
		println!("{:?}", self);
		database::get_database().replace_one(database::COLLECTION_ALBUMS, &self.id, self)
	}

	fn delete(&self) -> Result<()> {
		database::get_database().delete_one(database::COLLECTION_ALBUMS, &self.id)
	}
}

impl DatabaseUserEntity for Album {
	fn get_as_user(id: &str, user_id: i64) -> Result<Option<Self>>{
		match Self::get(id) {
			Some(album) => {
				if album.user_id != user_id {
					Err(Box::from(format!("User {} does not have access to album {}", user_id, album.id)))
				} else {
					Ok(Some(album))
				}
			}
			None => Ok(None)
		}
	}

	fn get_all_as_user(user_id: i64) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_ALBUMS, Some(user_id), None, None) 
	}

	fn get_all_with_ids_as_user(ids: &[&str], user_id: i64) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_ALBUMS, Some(user_id), Some(ids), None) 
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
			public: false,
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