use crate::entities::Session;
use serde::{Serialize, Deserialize};

use crate::ids;
use crate::database;
use crate::database::*;
use crate::error::*;
use crate::entities::AccessControl;
use crate::entities::collection::Collection;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	#[serde(default)]
	pub id: String,
	pub user_id: String,
	pub title: String,
	#[serde(default)]
	pub thumb_photo_id: Option<String>,
	#[serde(default)]
	pub photos: Vec<String>,

	pub tags: Vec<String>
}

impl Album {
	pub fn new(user_id: String, title: &str) -> Self {
		let id = ids::create_unique_id();

		Self {
			id,
			user_id,
			title: title.to_string(),
			thumb_photo_id: None,
			photos: vec!{},
			tags: vec!{}
		}
	}

	/// Get all collections that this album is part of.
	pub fn get_collections(&self) -> Result<Vec<Collection>> {
		database::get_database().get_collections_with_album(&self.id)
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

impl DatabaseEntityBatch for Album {
	fn get_with_ids(ids: &[&str]) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_ALBUMS, None, Some(ids), None)
	}
}

impl DatabaseUserEntity for Album {
	fn get_as_user(id: &str, user_id: String) -> Result<Option<Self>>{
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

	fn get_all_as_user(user_id: String) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_ALBUMS, Some(&user_id), None, None)
	}

	fn get_all_with_ids_as_user(ids: &[&str], user_id: String) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_ALBUMS, Some(&user_id), Some(ids), None)
	}
}

impl AccessControl for Album {
	fn can_view(&self, session: &Option<Session>) -> bool {
		// Check if user is owner of album
		if session_owns_album(self, session) {
			return true;
		}

		// Check if album is part of any collection that user has access to
		if let Ok(collections) = self.get_collections() {
			for collection in collections {
				if collection.can_view(session) {
					return true;
				}
			}
		}

		false
	}

    fn can_update(&self, session: &Option<Session>) -> bool {
		session_owns_album(self, session)
	}
}

/// Check if Album is owned by user of given session
fn session_owns_album(album: &Album, session_opt: &Option<Session>) -> bool {
	if let Some(session) = session_opt {
		if let Some(user_id) = &session.user_id {
			if &album.user_id == user_id {
				return true;
			}
		}
	}

	false
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ids::create_unique_id;


	#[test]
	fn new() {
		const TITLE: &str = "Hello world";
		const USER_ID: &str = "100";

		let album = Album::new(USER_ID.to_string(), TITLE);

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
	fn can_view() {
		let session_owner = create_dummy_session(true);
		// let session_not_owner = create_dummy_session(true);
		// let session_anonymous = create_dummy_session(false);

		let mut album = create_dummy_album_with_id("");
		album.user_id = session_owner.user_id.to_owned().unwrap();

		// Only the user that owns the album may access it
		assert_eq!(album.can_view(&Some(session_owner)), true);
		// TODO: can't test this anymore without DB connection
		// assert_eq!(album.can_view(&Some(session_not_owner)), false);
		// assert_eq!(album.can_view(&Some(session_anonymous)), false);
		// assert_eq!(album.can_view(&None), false);
	}

	#[test]
	fn can_update() {
		let session_owner = create_dummy_session(true);
		let session_not_owner = create_dummy_session(true);
		let session_anonymous = create_dummy_session(false);

		let mut album = create_dummy_album_with_id("");
		album.user_id = session_owner.user_id.to_owned().unwrap();

		// Only the user that owns the album may access it
		assert_eq!(album.can_update(&Some(session_owner)), true);
		assert_eq!(album.can_update(&Some(session_not_owner)), false);
		assert_eq!(album.can_update(&Some(session_anonymous)), false);
		assert_eq!(album.can_update(&None), false);
	}

	fn create_dummy_album_with_id(id: &str) -> Album {
		Album{
			id: id.to_string(),
			user_id: create_unique_id(),
			title: "title".to_string(),
			thumb_photo_id: Some(bson::oid::ObjectId::new().to_hex()),
			photos: vec!{
				bson::oid::ObjectId::new().to_hex(),
				bson::oid::ObjectId::new().to_hex(),
				bson::oid::ObjectId::new().to_hex()
			},
			tags: vec!{}
		}
	}

	fn create_dummy_session(with_user: bool) -> Session {
		let mut session = Session::new();

		if with_user {
			session.user_id = Some(create_unique_id());
		}

		session
	}
}