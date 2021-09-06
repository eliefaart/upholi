use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use crate::database;
use crate::database::{Database, DatabaseEntity};
use upholi_lib::ids::create_unique_id;
use crate::error::*;

/// A client session
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
	pub id: String,
	pub user_id: Option<String>,
	created_on: chrono::DateTime<Utc>,

	/// List of tokens (not fixed IDs) of collections that this session is authenticated to access
	pub authenticated_for_collection_tokens: Vec<String>,
}

impl Session {
	pub fn new() -> Self {
		Self {
			id: create_unique_id(),
			user_id: None,
			created_on: Utc::now(),
			authenticated_for_collection_tokens: vec!{}
		}
	}

	pub fn set_user(&mut self, user_id: &str) {
		self.user_id = Some(user_id.to_string());
	}
}

impl DatabaseEntity for Session {
	fn get(id: &str) -> Result<Option<Self>> {
		database::get_database().find_one(database::COLLECTION_SESSIONS, id)
	}

	fn insert(&self) -> Result<()> {
		database::get_database().insert_one(database::COLLECTION_SESSIONS, &self)?;
		Ok(())
	}

	fn update(&self)  -> Result<()> {
		database::get_database().replace_one(database::COLLECTION_SESSIONS, &self.id, self)
	}

	fn delete(&self)  -> Result<()> {
		database::get_database().delete_one(database::COLLECTION_SESSIONS, &self.id)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		let session = Session::new();

		assert!(session.id.len() != 0);
		assert!(session.user_id.is_none());
	}

	#[test]
	fn set_user() {
		const USER_ID: &str = "99995555";

		let mut session = Session::new();
		session.set_user(USER_ID);

		assert!(session.user_id.is_some());
		assert_eq!(session.user_id.unwrap(), USER_ID);
	}
}