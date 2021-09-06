use crate::database::DatabaseExt;
use crate::storage::init_storage_for_user;
use serde::{Serialize,Deserialize};
use crate::error::*;
use crate::database;
use crate::database::{Database, DatabaseEntity};
use upholi_lib::ids::create_unique_id;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	pub id: String,
	pub username: String,
	pub public_key: String,
}

impl User {
	pub async fn create(username: String, public_key: String) -> Result<User> {
		let user = User{
			id: create_unique_id(),
			username,
			public_key
		};

		user.insert()?;
		init_storage_for_user(&user).await?;
		Ok(user)
	}

	pub async fn get_by_username(username: &str) -> Result<Option<User>> {
		database::get_database().get_user_by_username(username)
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