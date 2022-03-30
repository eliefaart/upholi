use crate::database::{DatabaseEntity, DatabaseEntityUserOwned};
use crate::error::*;
use async_trait::async_trait;
use upholi_lib::http::request::EntityAuthorizationProof;
use upholi_lib::http::response::Album;
use upholi_lib::ids;
use upholi_lib::models::EncryptedAlbum;

use super::session::Session;
use super::{session_owns_entity, AccessControl, UserEntity};

pub type DbAlbum = UserEntity<EncryptedAlbum>;

impl From<DbAlbum> for Album {
	fn from(entity: DbAlbum) -> Self {
		Self {
			id: entity.id,
			user_id: entity.user_id,
			data: entity.entity.data,
			key: entity.entity.key,
		}
	}
}

impl DbAlbum {
	pub fn from(album: EncryptedAlbum, user_id: &str) -> Self {
		Self {
			id: ids::create_unique_id(),
			user_id: user_id.to_string(),
			entity: album,
		}
	}
}

#[async_trait]
impl DatabaseEntity for DbAlbum {
	async fn get(id: &str) -> Result<Option<Self>> {
		super::super::find_one(super::super::COLLECTION_ALBUMS, id, None).await
	}

	async fn insert(&self) -> Result<()> {
		super::super::insert_one(super::super::COLLECTION_ALBUMS, self).await?;
		Ok(())
	}

	async fn update(&self) -> Result<()> {
		super::super::replace_one(super::super::COLLECTION_ALBUMS, &self.id, self).await
	}

	async fn delete(&self) -> Result<()> {
		super::super::delete_one(super::super::COLLECTION_ALBUMS, &self.id).await
	}
}

#[async_trait]
impl DatabaseEntityUserOwned for DbAlbum {
	async fn get_for_user(id: &str, user_id: String) -> Result<Option<Self>> {
		match Self::get(id).await? {
			Some(album) => {
				if album.user_id != user_id {
					Err(Box::from(format!("User {} does not have access to album {}", user_id, album.id)))
				} else {
					Ok(Some(album))
				}
			}
			None => Ok(None),
		}
	}

	async fn get_all_for_user(user_id: String) -> Result<Vec<Self>> {
		super::super::find_many(super::super::COLLECTION_ALBUMS, Some(&user_id), None, None, None).await
	}

	async fn get_many_for_user(ids: &[&str], user_id: String) -> Result<Vec<Self>> {
		super::super::find_many(super::super::COLLECTION_ALBUMS, Some(&user_id), Some(ids), None, None).await
	}
}

impl AccessControl for DbAlbum {
	fn can_view(&self, session: &Option<Session>, proof: Option<EntityAuthorizationProof>) -> bool {
		// Check if user is owner of album
		if session_owns_entity(self, session) {
			true
		} else if let Some(proof) = proof {
			proof.key_hash == self.entity.key_hash
		} else {
			false
		}
	}

	fn can_update(&self, session: &Option<Session>) -> bool {
		session_owns_entity(self, session)
	}
}

#[cfg(test)]
mod tests {
	use crate::database::models::album::DbAlbum;
	use upholi_lib::{http::response::Album, models::EncryptedAlbum, EncryptedData};

	#[test]
	fn no_env_vars() {
		let user_id = "user_id";
		let key_hash = "key_hash";

		let album = EncryptedAlbum {
			data: EncryptedData {
				base64: String::new(),
				nonce: String::new(),
				format_version: 1,
			},
			key: EncryptedData {
				base64: String::new(),
				nonce: String::new(),
				format_version: 1,
			},
			key_hash: String::from(key_hash),
		};

		let db_album = DbAlbum::from(album, user_id);
		assert_eq!(db_album.entity.key_hash, key_hash);

		let album: Album = db_album.into();
		assert_eq!(album.user_id, user_id);
	}
}
