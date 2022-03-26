use super::session::Session;
use super::session_owns_entity;
use super::AccessControl;
use super::UserEntity;
use crate::database::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use upholi_lib::http::request::EntityAuthorizationProof;
use upholi_lib::http::request::FindSharesFilter;
use upholi_lib::http::request::UpsertShare;
use upholi_lib::http::response::Share;
use upholi_lib::ids;
use upholi_lib::ids::create_unique_id;
use upholi_lib::EncryptedData;
use upholi_lib::ShareType;

pub type DbShare = UserEntity<UpsertShare>;

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Share {
// 	pub id: String,
// 	pub user_id: String,
// 	pub identifier_hash: String,
// 	pub type_: ShareType,
// 	pub password: EncryptedData,
// 	pub data: EncryptedData,
// 	pub key: EncryptedData,
// }

impl From<DbShare> for Share {
	fn from(db_share: DbShare) -> Self {
		Self {
			id: db_share.id,
			user_id: db_share.user_id,
			identifier_hash: db_share.entity.identifier_hash,
			type_: db_share.entity.type_,
			password: db_share.entity.password,
			data: db_share.entity.data,
			key: db_share.entity.key,
		}
	}
}

impl DbShare {
	pub fn from(share: UpsertShare, user_id: &str) -> Self {
		Self {
			id: ids::create_unique_id(),
			user_id: user_id.to_string(),
			entity: share,
		}
	}

	pub async fn find_shares(user_id: &str, filters: FindSharesFilter) -> Result<Vec<Self>> {
		super::super::find_shares(user_id, filters).await
	}
}

// impl From<UpsertShare> for DbShare {
// 	fn from(source: UpsertShare) -> Self {
// 		Self {
// 			id: create_unique_id(),
// 			user_id: String::new(),
// 			identifier_hash: source.identifier_hash,
// 			type_: source.type_,
// 			password: source.password,
// 			data: source.data,
// 			key: source.key,
// 		}
// 	}
// }

#[async_trait]
impl DatabaseEntity for DbShare {
	async fn get(id: &str) -> Result<Option<Self>> {
		super::super::find_one(super::super::COLLECTION_SHARES, id, None).await
	}

	async fn insert(&self) -> Result<()> {
		super::super::insert_one(super::super::COLLECTION_SHARES, self).await?;
		Ok(())
	}

	async fn update(&self) -> Result<()> {
		super::super::replace_one(super::super::COLLECTION_SHARES, &self.id, self).await
	}

	async fn delete(&self) -> Result<()> {
		super::super::delete_one(super::super::COLLECTION_SHARES, &self.id).await
	}
}

#[async_trait]
impl DatabaseEntityBatch for DbShare {
	async fn get_many(ids: &[&str]) -> Result<Vec<Self>> {
		super::super::find_many(super::super::COLLECTION_SHARES, None, Some(ids), None, None).await
	}

	async fn delete_many(ids: &[&str]) -> Result<()> {
		super::super::delete_many(super::super::COLLECTION_SHARES, ids).await
	}
}

#[async_trait]
impl DatabaseEntityUserOwned for DbShare {
	async fn get_for_user(id: &str, user_id: String) -> Result<Option<Self>> {
		match Self::get(id).await? {
			Some(share) => {
				if share.user_id != user_id {
					Err(Box::from(format!("User {} does not have access to share {}", user_id, share.id)))
				} else {
					Ok(Some(share))
				}
			}
			None => Ok(None),
		}
	}

	async fn get_all_for_user(user_id: String) -> Result<Vec<Self>> {
		super::super::find_many(super::super::COLLECTION_SHARES, Some(&user_id), None, None, None).await
	}

	async fn get_many_for_user(ids: &[&str], user_id: String) -> Result<Vec<Self>> {
		super::super::find_many(super::super::COLLECTION_SHARES, Some(&user_id), Some(ids), None, None).await
	}
}

impl AccessControl for DbShare {
	fn can_view(&self, _session: &Option<Session>, _proof: Option<EntityAuthorizationProof>) -> bool {
		true
	}

	fn can_update(&self, session: &Option<Session>) -> bool {
		session_owns_entity(self, session)
	}
}
