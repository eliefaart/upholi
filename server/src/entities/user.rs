use crate::database::DatabaseExt;
use crate::storage::init_storage_for_user;
use serde::{Serialize,Deserialize};
use upholi_lib::EncryptedData;
use upholi_lib::passwords::{hash_password, verify_password_hash};
use crate::error::*;
use crate::database;
use crate::database::{Database, DatabaseEntity};
use upholi_lib::ids::create_unique_id;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	pub id: String,
	pub username: String,
	pub password_phc: String,
	pub key: EncryptedData,
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
			key
		};

		user.insert()?;
		init_storage_for_user(&user).await?;
		Ok(user)
	}

	pub async fn get_by_username(username: &str) -> Result<Option<User>> {
		database::get_database().get_user_by_username(username)
	}

	pub fn password_valid(&self, password: &str) -> bool {
		verify_password_hash(password, &self.password_phc)
	}
}

impl DatabaseEntity for User {
	fn get(id: &str) -> Result<Option<Self>> {
		database::get_database().find_one(database::COLLECTION_USERS, id)
	}

	fn insert(&self) -> Result<()> {
		database::get_database().insert_one(database::COLLECTION_USERS, &self)?;
		Ok(())
	}

	fn update(&self)  -> Result<()> {
		database::get_database().replace_one(database::COLLECTION_USERS, &self.id, self)
	}

	fn delete(&self)  -> Result<()> {
		database::get_database().delete_one(database::COLLECTION_USERS, &self.id)
	}
}