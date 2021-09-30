use crate::entities::Session;
use serde::{Serialize, Deserialize};
use upholi_lib::EncryptedData;
use upholi_lib::ShareType;
use upholi_lib::http::request::CreateShare;
use upholi_lib::http::request::EntityAuthorizationProof;
use upholi_lib::ids::create_unique_id;

use crate::database;
use crate::database::*;
use crate::error::*;
use crate::entities::AccessControl;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Share {
	pub id: String,
	pub user_id: String,
	pub type_: ShareType,
	pub password: EncryptedData,
	pub data: EncryptedData,
	pub key: EncryptedData
}

impl From<CreateShare> for Share {
	fn from(source: CreateShare) -> Self {
		Self {
			id: create_unique_id(),
			user_id: String::new(),
			type_: source.type_,
			password: source.password,
			data: source.data,
			key: source.key
		}
	}
}

impl DatabaseEntity for Share {
	fn get(id: &str) -> Result<Option<Self>> {
		database::get_database().find_one(database::COLLECTION_SHARES, id)
	}

	fn insert(&self) -> Result<()> {
		if self.id.is_empty() {
			return Err(Box::from(EntityError::IdMissing));
		}

		match Self::get(&self.id)? {
			Some(_) => Err(Box::from(EntityError::AlreadyExists)),
			None => {
				database::get_database().insert_one(database::COLLECTION_SHARES, &self)?;
				Ok(())
			}
		}
	}

	fn update(&self) -> Result<()> {
		database::get_database().replace_one(database::COLLECTION_SHARES, &self.id, self)
	}

	fn delete(&self) -> Result<()> {
		database::get_database().delete_one(database::COLLECTION_SHARES, &self.id)
	}
}

impl DatabaseEntityBatch for Share {
	fn get_with_ids(ids: &[&str]) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_SHARES, None, Some(ids), None)
	}
}

impl DatabaseUserEntity for Share {
	fn get_as_user(id: &str, user_id: String) -> Result<Option<Self>>{
		match Self::get(id)? {
			Some(share) => {
				if share.user_id != user_id {
					Err(Box::from(format!("User {} does not have access to share {}", user_id, share.id)))
				} else {
					Ok(Some(share))
				}
			}
			None => Ok(None)
		}
	}

	fn get_all_as_user(user_id: String) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_SHARES, Some(&user_id), None, None)
	}

	fn get_all_with_ids_as_user(ids: &[&str], user_id: String) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_SHARES, Some(&user_id), Some(ids), None)
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