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