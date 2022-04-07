use crate::database::DatabaseEntity;
use crate::error::*;
use crate::storage::init_storage_for_user;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use upholi_lib::http::response::UserInfo;
use upholi_lib::ids::create_unique_id;
use upholi_lib::passwords::{hash_password, verify_password_hash};
use upholi_lib::EncryptedData;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	pub id: String,
	pub username: String,
	pub password_phc: String,
	pub key: EncryptedData,
}

impl Into<UserInfo> for User {
	fn into(self) -> UserInfo {
		UserInfo {
			id: self.id,
			username: self.username,
			key: self.key,
		}
	}
}

impl User {
	pub async fn create(username: String, password: String, key: EncryptedData) -> Result<User> {
		let user_id = create_unique_id();

		let salt = create_unique_id();
		let password_phc = hash_password(&password, &salt)?;

		let user = User {
			id: user_id,
			username,
			password_phc,
			key,
		};

		user.insert().await?;
		init_storage_for_user(&user).await?;
		Ok(user)
	}

	pub async fn get_by_username(username: &str) -> Result<Option<User>> {
		super::super::get_user_by_username(username).await
	}

	pub fn password_valid(&self, password: &str) -> bool {
		verify_password_hash(password, &self.password_phc)
	}
}

#[async_trait]
impl DatabaseEntity for User {
	async fn get(id: &str) -> Result<Option<Self>> {
		super::super::find_one(super::super::COLLECTION_USERS, id, None).await
	}

	async fn insert(&self) -> Result<()> {
		super::super::insert_one(super::super::COLLECTION_USERS, self).await?;
		Ok(())
	}

	async fn update(&self) -> Result<()> {
		super::super::replace_one(super::super::COLLECTION_USERS, &self.id, self).await
	}

	async fn delete(&self) -> Result<()> {
		super::super::delete_one(super::super::COLLECTION_USERS, &self.id).await
	}
}
