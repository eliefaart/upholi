use crate::database::*;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use upholi_lib::http::request::EntityAuthorizationProof;
use upholi_lib::http::request::FindSharesFilter;
use upholi_lib::http::request::UpsertShare;
use upholi_lib::ids::create_unique_id;
use upholi_lib::EncryptedData;
use upholi_lib::ShareType;

use super::session::Session;
use super::AccessControl;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Share {
	pub id: String,
	pub user_id: String,
	pub identifier_hash: String,
	pub type_: ShareType,
	pub password: EncryptedData,
	pub data: EncryptedData,
	pub key: EncryptedData,
}

impl Share {
	pub async fn find_shares(user_id: &str, filters: FindSharesFilter) -> Result<Vec<Self>> {
		super::super::find_shares(user_id, filters).await
	}
}

impl From<UpsertShare> for Share {
	fn from(source: UpsertShare) -> Self {
		Self {
			id: create_unique_id(),
			user_id: String::new(),
			identifier_hash: source.identifier_hash,
			type_: source.type_,
			password: source.password,
			data: source.data,
			key: source.key,
		}
	}
}

#[async_trait]
impl DatabaseEntity for Share {
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
impl DatabaseEntityBatch for Share {
	async fn get_many(ids: &[&str]) -> Result<Vec<Self>> {
		super::super::find_many(super::super::COLLECTION_SHARES, None, Some(ids), None, None).await
	}

	async fn delete_many(ids: &[&str]) -> Result<()> {
		super::super::delete_many(super::super::COLLECTION_SHARES, ids).await
	}
}

#[async_trait]
impl DatabaseEntityUserOwned for Share {
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

impl AccessControl for Share {
	fn can_view(&self, _session: &Option<Session>, _proof: Option<EntityAuthorizationProof>) -> bool {
		true
	}

	fn can_update(&self, session: &Option<Session>) -> bool {
		session_owns_share(self, session)
	}
}

/// Check if Share is owned by user of given session
fn session_owns_share(share: &Share, session_opt: &Option<Session>) -> bool {
	if let Some(session) = session_opt {
		if let Some(user_id) = &session.user_id {
			if &share.user_id == user_id {
				return true;
			}
		}
	}

	false
}
