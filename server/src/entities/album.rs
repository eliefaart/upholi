use crate::entities::Session;
use serde::{Serialize, Deserialize};
use upholi_lib::EncryptedData;
use upholi_lib::http::request::CreateAlbum;
use upholi_lib::http::request::EntityAuthorizationProof;
use upholi_lib::ids::create_unique_id;

use crate::database;
use crate::database::*;
use crate::error::*;
use crate::entities::AccessControl;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	pub id: String,
	pub user_id: String,
	pub data: EncryptedData,
	pub key: EncryptedData,
	pub key_hash: String
}

impl From<CreateAlbum> for Album {
	fn from(source: CreateAlbum) -> Self {
		Self {
			id: create_unique_id(),
			user_id: String::new(),
			data: source.data,
			key: source.key,
			key_hash: source.key_hash
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
	fn can_view(&self, session: &Option<Session>, proof: Option<EntityAuthorizationProof>) -> bool {
		// Check if user is owner of album
		if session_owns_album(self, session) {
			true
		}
		else {
			if let Some(proof) = proof {
				proof.key_hash == self.key_hash
			}
			else {
				false
			}
		}
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