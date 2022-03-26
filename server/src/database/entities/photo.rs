use super::{session::Session, AccessControl};
use super::{session_owns_entity, UserEntity};
use crate::database::{DatabaseEntityMinimal, ProjectField};
use crate::error::*;
use crate::{
	database::{self, DatabaseEntity, DatabaseEntityBatch, DatabaseEntityUserOwned},
	error::EntityError,
};
use async_trait::async_trait;
use upholi_lib::http::request::{EntityAuthorizationProof, UploadPhoto};
use upholi_lib::http::response::{Photo, PhotoMinimal};
use upholi_lib::ids::{self};

pub type DbPhoto = UserEntity<UploadPhoto>;

impl From<DbPhoto> for Photo {
	fn from(photo: DbPhoto) -> Self {
		Self {
			id: photo.id,
			user_id: photo.user_id,
			hash: photo.entity.hash,
			width: photo.entity.width as i32,
			height: photo.entity.height as i32,
			data: photo.entity.data,
			key: photo.entity.key,
			thumbnail_nonce: photo.entity.thumbnail_nonce,
			preview_nonce: photo.entity.preview_nonce,
			original_nonce: photo.entity.original_nonce,
		}
	}
}

impl DbPhoto {
	pub fn from(photo: UploadPhoto, user_id: &str) -> Self {
		Self {
			id: ids::create_unique_id(),
			user_id: user_id.to_string(),
			entity: photo,
		}
	}
}

impl DbPhoto {
	pub async fn hash_exists_for_user(user_id: &str, hash: &str) -> Result<bool> {
		super::super::photo_exists_for_user(user_id, hash).await
	}

	/// Get the fields contained in the PhotoMinimal struct
	fn get_fields_minimal<'a>() -> Vec<ProjectField<'a>> {
		vec![
			ProjectField { path: "id", name: "id" },
			ProjectField {
				path: "entity.width",
				name: "width",
			},
			ProjectField {
				path: "entity.height",
				name: "height",
			},
		]
	}
}

#[async_trait]
impl DatabaseEntity for DbPhoto {
	async fn get(id: &str) -> Result<Option<Self>> {
		super::super::find_one(super::super::COLLECTION_PHOTOS, id, None).await
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
impl DatabaseEntityMinimal for DbPhoto {
	type TMinimal = PhotoMinimal;

	async fn get_minimal(id: &str) -> Result<Option<Self::TMinimal>> {
		super::super::find_one(super::super::COLLECTION_PHOTOS, id, Some(Self::get_fields_minimal())).await
	}

	async fn get_many_minimal(ids: &[&str]) -> Result<Vec<Self::TMinimal>> {
		super::super::find_many(
			super::super::COLLECTION_PHOTOS,
			None,
			Some(ids),
			None,
			Some(Self::get_fields_minimal()),
		)
		.await
	}

	async fn get_all_for_user_minimal(user_id: String) -> Result<Vec<Self::TMinimal>> {
		let sort = database::SortField {
			field: "createdOn",
			ascending: false,
		};
		super::super::find_many(
			super::super::COLLECTION_PHOTOS,
			Some(&user_id),
			None,
			Some(&sort),
			Some(Self::get_fields_minimal()),
		)
		.await
	}
}

#[async_trait]
impl DatabaseEntityBatch for DbPhoto {
	async fn get_many(ids: &[&str]) -> Result<Vec<Self>> {
		super::super::find_many(super::super::COLLECTION_PHOTOS, None, Some(ids), None, None).await
	}

	async fn delete_many(ids: &[&str]) -> Result<()> {
		super::super::delete_many(super::super::COLLECTION_PHOTOS, ids).await
	}
}

#[async_trait]
impl DatabaseEntityUserOwned for DbPhoto {
	async fn get_for_user(id: &str, user_id: String) -> Result<Option<Self>> {
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

	async fn get_all_for_user(user_id: String) -> Result<Vec<Self>> {
		let sort = database::SortField {
			field: "createdOn",
			ascending: false,
		};
		super::super::find_many(super::super::COLLECTION_PHOTOS, Some(&user_id), None, Some(&sort), None).await
	}

	async fn get_many_for_user(ids: &[&str], user_id: String) -> Result<Vec<Self>> {
		let sort = database::SortField {
			field: "createdOn",
			ascending: false,
		};
		super::super::find_many(super::super::COLLECTION_PHOTOS, Some(&user_id), Some(ids), Some(&sort), None).await
	}
}

impl AccessControl for DbPhoto {
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
