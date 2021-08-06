use upholi_lib::http::request::UploadPhoto;
use upholi_lib::http::*;
use serde::{Deserialize, Serialize};

use crate::database::{Database, DatabaseExt};
use crate::{error::*, ids};
use crate::{database::{self, DatabaseEntity, DatabaseEntityBatch, DatabaseUserEntity}, error::EntityError};

use super::{AccessControl, session::Session};

// Maybe a wrapper struct? {id: string, info: T}???
// Then I can re-use the other struct which is identical.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	pub id: String,
	/// Owner user id
	pub user_id: String,
	pub width: i32,
	pub height: i32,
	/// Encrypted data, contains width, height, exif, etc
	pub data: EncryptedData,
	pub data_version: i32,
	/// Key that all data and file bytes of this photo is encrypted with. This key is encrypted with the owner's private key.
	pub key: EncryptedData,
	pub share_keys: Vec<ShareKey>,
	pub thumbnail_nonce: String,
	pub preview_nonce: String,
	pub original_nonce: String
}

impl From<UploadPhoto> for Photo {
	fn from(source: UploadPhoto) -> Self {
		Self {
			id: ids::create_unique_id(),
			user_id: String::new(),
			width: source.width as i32,
			height: source.height as i32,
			data: source.data,
			data_version: source.data_version as i32,
			key: source.key,
			share_keys: source.share_keys,
			thumbnail_nonce: source.thumbnail_nonce,
			preview_nonce: source.preview_nonce,
			original_nonce: source.original_nonce,
		}
	}
}

impl DatabaseEntity for Photo {
	fn get(id: &str) -> Result<Option<Self>> {
		database::get_database().find_one(database::COLLECTION_PHOTOS, id)
	}

	fn insert(&self) -> Result<()> {
		if self.id.is_empty() {
			return Err(Box::from(EntityError::IdMissing));
		}

		match Self::get(&self.id)? {
			Some(_) => Err(Box::from(EntityError::AlreadyExists)),
			None => {
				database::get_database().insert_one(database::COLLECTION_PHOTOS, &self)?;
				Ok(())
			}
		}
	}

	fn update(&self) -> Result<()> {
		database::get_database().replace_one(database::COLLECTION_PHOTOS, &self.id, self)
	}

	fn delete(&self) -> Result<()> {
		database::get_database().delete_one(database::COLLECTION_PHOTOS, &self.id)
	}
}

impl DatabaseEntityBatch for Photo {
	fn get_with_ids(ids: &[&str]) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_PHOTOS, None, Some(ids), None)
	}
}

impl DatabaseUserEntity for Photo {
	fn get_as_user(id: &str, user_id: String) -> Result<Option<Self>>{
		match Self::get(id)? {
			Some(photo) => {
				if photo.user_id != user_id {
					Err(Box::from(EntityError::NoAccess))
				} else {
					Ok(Some(photo))
				}
			},
			None => Ok(None)
		}
	}

	fn get_all_as_user(user_id: String) -> Result<Vec<Self>> {
		let sort = database::SortField{
			field: "createdOn",
			ascending: false
		};
		database::get_database().find_many(database::COLLECTION_PHOTOS, Some(&user_id), None, Some(&sort))
	}

	fn get_all_with_ids_as_user(ids: &[&str], user_id: String) -> Result<Vec<Self>> {
		let sort = database::SortField{
			field: "createdOn",
			ascending: false
		};
		database::get_database().find_many(database::COLLECTION_PHOTOS, Some(&user_id), Some(ids), Some(&sort))
	}
}

impl AccessControl for Photo {
	fn can_view(&self, session: &Option<Session>) -> bool {
		if session_owns_photo(self, session) {
			return true;
		}

		// Check if photo is part of any collection,
		// if so, photo is publically accessible too.
		if let Ok(albums) = database::get_database().get_albums_with_photo(&self.id) {
			for album in albums {
				if album.can_view(session) {
					return true;
				}
			}
		}

		false
	}

    fn can_update(&self, session: &Option<Session>) -> bool {
		session_owns_photo(self, session)
	}
}

/// Check if Photo is owned by user of given session
fn session_owns_photo(photo: &Photo, session_opt: &Option<Session>) -> bool {
	if let Some(session) = session_opt {
		if let Some(user_id) = &session.user_id {
			if &photo.user_id == user_id {
				return true;
			}
		}
	}

	false
}