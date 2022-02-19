use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use upholi_lib::http::request::CreateAlbum;
use upholi_lib::http::request::EntityAuthorizationProof;
use upholi_lib::ids::create_unique_id;
use upholi_lib::EncryptedData;

use super::session::Session;
use super::AccessControl;
use crate::database::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	pub id: String,
	pub user_id: String,
	pub data: EncryptedData,
	pub key: EncryptedData,
	pub key_hash: String,
}

impl From<CreateAlbum> for Album {
	fn from(source: CreateAlbum) -> Self {
		Self {
			id: create_unique_id(),
			user_id: String::new(),
			data: source.data,
			key: source.key,
			key_hash: source.key_hash,
		}
	}
}

#[async_trait]
impl DatabaseEntity for Album {
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
impl DatabaseEntityBatch for Album {
	async fn get_many(ids: &[&str]) -> Result<Vec<Self>> {
		super::super::find_many(super::super::COLLECTION_ALBUMS, None, Some(ids), None, None).await
	}
}

#[async_trait]
impl DatabaseEntityUserOwned for Album {
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

impl AccessControl for Album {
	fn can_view(&self, session: &Option<Session>, proof: Option<EntityAuthorizationProof>) -> bool {
		// Check if user is owner of album
		if session_owns_album(self, session) {
			true
		} else if let Some(proof) = proof {
			proof.key_hash == self.key_hash
		} else {
			false
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
