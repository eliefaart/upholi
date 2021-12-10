use crate::error::*;
use crate::{
	database::{self, DatabaseEntity, DatabaseEntityBatch, DatabaseUserEntity},
	error::EntityError,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use upholi_lib::http::request::{EntityAuthorizationProof, UploadPhoto};
use upholi_lib::ids::create_unique_id;
use upholi_lib::EncryptedData;

use super::{session::Session, AccessControl};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	pub id: String,
	/// Hash of original photo file
	pub hash: String,
	/// Owner user id
	pub user_id: String,
	pub width: i32,
	pub height: i32,
	/// Encrypted data, contains width, height, exif, etc
	pub data: EncryptedData,
	pub key: EncryptedData,
	pub key_hash: String,
	pub thumbnail_nonce: String,
	pub preview_nonce: String,
	pub original_nonce: String,
}

impl From<UploadPhoto> for Photo {
	fn from(source: UploadPhoto) -> Self {
		Self {
			id: create_unique_id(),
			hash: source.hash,
			user_id: String::new(),
			width: source.width as i32,
			height: source.height as i32,
			data: source.data,
			key: source.key,
			key_hash: source.key_hash,
			thumbnail_nonce: source.thumbnail_nonce,
			preview_nonce: source.preview_nonce,
			original_nonce: source.original_nonce,
		}
	}
}

impl Photo {
	pub async fn hash_exists_for_user(user_id: &str, hash: &str) -> Result<bool> {
		super::super::photo_exists_for_user(user_id, hash).await
	}
}

#[async_trait]
impl DatabaseEntity for Photo {
	async fn get(id: &str) -> Result<Option<Self>> {
		super::super::find_one(super::super::COLLECTION_PHOTOS, id).await
	}

	async fn insert(&self) -> Result<()> {
		super::super::insert_one(super::super::COLLECTION_PHOTOS, self).await?;
		Ok(())
	}

	async fn update(&self) -> Result<()> {
		super::super::replace_one(super::super::COLLECTION_PHOTOS, &self.id, self).await
	}

	async fn delete(&self) -> Result<()> {
		super::super::delete_one(super::super::COLLECTION_PHOTOS, &self.id).await
	}
}

#[async_trait]
impl DatabaseEntityBatch for Photo {
	async fn get_with_ids(ids: &[&str]) -> Result<Vec<Self>> {
		super::super::find_many(super::super::COLLECTION_PHOTOS, None, Some(ids), None).await
	}
}

#[async_trait]
impl DatabaseUserEntity for Photo {
	async fn get_as_user(id: &str, user_id: String) -> Result<Option<Self>> {
		match Self::get(id).await? {
			Some(photo) => {
				if photo.user_id != user_id {
					Err(Box::from(EntityError::NoAccess))
				} else {
					Ok(Some(photo))
				}
			}
			None => Ok(None),
		}
	}

	async fn get_all_as_user(user_id: String) -> Result<Vec<Self>> {
		let sort = database::SortField {
			field: "createdOn",
			ascending: false,
		};
		super::super::find_many(super::super::COLLECTION_PHOTOS, Some(&user_id), None, Some(&sort)).await
	}

	async fn get_all_with_ids_as_user(ids: &[&str], user_id: String) -> Result<Vec<Self>> {
		let sort = database::SortField {
			field: "createdOn",
			ascending: false,
		};
		super::super::find_many(super::super::COLLECTION_PHOTOS, Some(&user_id), Some(ids), Some(&sort)).await
	}
}

impl AccessControl for Photo {
	fn can_view(&self, session: &Option<Session>, proof: Option<EntityAuthorizationProof>) -> bool {
		// Check if user is owner of album
		if session_owns_photo(self, session) {
			true
		} else {
			if let Some(proof) = proof {
				proof.key_hash == self.key_hash
			} else {
				false
			}
		}
	}

	fn can_update(&self, session: &Option<Session>) -> bool {
		session_owns_photo(self, session)
	}
}

/// Check if Photo is owned by user of given session
fn session_owns_photo(photo: &Photo, session: &Option<Session>) -> bool {
	if let Some(session) = session {
		if let Some(user_id) = &session.user_id {
			if &photo.user_id == user_id {
				return true;
			}
		}
	}

	false
}
