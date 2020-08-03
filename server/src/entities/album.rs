use serde::{Serialize, Deserialize};

use crate::ids;
use crate::database;
use crate::database::*;
use crate::error::*;
use crate::entities::AccessControl;
use crate::entities::user::User;

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
	fn get(id: &str) -> Result<Option<Self>> {
		database::get_database().find_one(database::COLLECTION_ALBUMS, id)
	}

	fn insert(&self) -> Result<()> {
		if self.id.is_empty() {
			return Err(Box::from(EntityError::IdMissing));
		}

		match Self::get(&self.id)? {
			Some(_) => Err(Box::from(EntityError::AlreadyExists)),
			None => {
				database::get_database().insert_one(database::COLLECTION_ALBUMS, &self)?;
				Ok(())
			}
		}
	}

	fn update(&self) -> Result<()> {
		database::get_database().replace_one(database::COLLECTION_ALBUMS, &self.id, self)
	}

	fn delete(&self) -> Result<()> {
		database::get_database().delete_one(database::COLLECTION_ALBUMS, &self.id)
	}
}

impl DatabaseUserEntity for Album {
	fn get_as_user(id: &str, user_id: i64) -> Result<Option<Self>>{
		match Self::get(id)? {
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

impl AccessControl for Album {
	fn user_has_access(&self, user_opt: Option<User>) -> bool {
		if let Some(user) = user_opt {
			self.user_id == user.user_id || self.public
		} 
		else {
			self.public
		}
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

	#[test]
	fn access_private() {
		let user_album_owner = User{user_id: 1};
		let user_not_album_owner = User{user_id: 2};

		let mut album = create_dummy_album_with_id("");
		album.public = false;
		album.user_id = user_album_owner.user_id;

		// Only the user that owns the album may access it
		assert_eq!(album.user_has_access(Some(user_album_owner)), true);
		assert_eq!(album.user_has_access(Some(user_not_album_owner)), false);
		assert_eq!(album.user_has_access(None), false);
	}

	#[test]
	fn access_public() {
		let user_album_owner = User{user_id: 1};
		let user_not_album_owner = User{user_id: 2};

		let mut album = create_dummy_album_with_id("");
		album.public = true;
		album.user_id = user_album_owner.user_id;

		// Everyone may access the album, because it is public
		assert_eq!(album.user_has_access(Some(user_album_owner)), true);
		assert_eq!(album.user_has_access(Some(user_not_album_owner)), true);
		assert_eq!(album.user_has_access(None), true);
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