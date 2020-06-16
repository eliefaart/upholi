use crate::database;
use crate::database::{DatabaseOperations};
use crate::ids;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

fn get_collection() -> mongodb::Collection {
	database::DATABASE.collection(database::COLLECTION_SESSIONS)
}

/// A client session
#[derive(Serialize, Deserialize)]
pub struct Session {
	pub id: String,
	pub user_id: Option<i64>,
	created_on: chrono::DateTime<Utc>,

	/// Contains data related to an oauth login attempt
	/// Such as: state id, PKCE tokens. 
	/// The value of this field will be None if login has completed
	pub _oauth: Option<()>
}

impl Session {
	fn new() -> Self {
		Self {
			id: ids::create_unique_id(),
			user_id: None,
			created_on: Utc::now(),
			_oauth: None
		}
	}

	pub fn set_user(&mut self, user_id: i64) -> Result<(), String> {
		self.user_id = Some(user_id);
		Self::update(self)
	}
}

impl DatabaseOperations for Session {
	fn get(id: &str) -> Option<Self> {
		let collection = get_collection();
		database::find_one(id, &collection)
	}

	fn create() -> Result<Self, String> {
		let session = Self::new();

		let collection = get_collection();
		database::insert_item(&collection, &session)?;

		Ok(session)
	}

	fn update(&self)  -> Result<(), String> {
		let collection = get_collection();
		database::replace_one(&self.id, &self, &collection)
	}

	fn delete(&self)  -> Result<(), String> {
		let collection = get_collection();
		match database::delete_one(&self.id, &collection) {
			Some(_) => Ok(()),
			None => Err("Failed to delete session".to_string())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		let session = Session::new();

		assert!(session.id.len() != 0);
	}

	#[test]
	fn set_user() {
		const USER_ID: i64 = 999555i64;

		let mut session = Session::new();
		session.set_user(USER_ID).unwrap();

		assert!(session.user_id.is_some());
		assert_eq!(session.user_id.unwrap(), USER_ID);
	}
}