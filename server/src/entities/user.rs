use crate::storage::init_storage_for_user;
use crate::ids::create_unique_id;
use serde::{Serialize,Deserialize};
use crate::error::*;
use crate::database;
use crate::database::{Database, DatabaseEntity};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	pub id: String,

	/// Name/ID of an identity provider
	pub identity_provider: String,

	/// ID of user at identity provider
	pub identity_provider_user_id: String,
}

impl User {
	pub async fn create(identity_provider: String, identity_provider_user_id: String) -> Result<User> {
		let user = User{
			id: create_unique_id(),
			identity_provider,
			identity_provider_user_id
		};

		user.insert()?;
		init_storage_for_user(&user).await?;
		Ok(user)
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