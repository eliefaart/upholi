use crate::database;
use crate::database::{DatabaseOperations};
use crate::ids;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;

/// A client session
#[derive(Serialize, Deserialize)]
pub struct Session {
	pub id: String,
	pub user_id: Option<i64>,
	created_on: chrono::DateTime<Utc>,

	/// Contains data related to an oauth login attempt
	/// Such as: state id, PKCE tokens. 
	/// The value of this field will be None if login has completed
	pub oauth: Option<OauthData>
}

/// Contains data related to an oauth login attempt
#[derive(Serialize, Deserialize)]
pub struct OauthData {
	pub state: String,
	pub pkce_verifier: String
}

impl Session {
	pub fn new() -> Self {
		Self {
			id: ids::create_unique_id(),
			user_id: None,
			created_on: Utc::now(),
			oauth: None
		}
	}

	pub fn set_user(&mut self, user_id: i64) {
		self.user_id = Some(user_id);
	}

	pub fn set_oauth_data(&mut self, state: &str, pkce_verifier: &str) {
		self.oauth = Some(OauthData{
			state: state.to_string(),
			pkce_verifier: pkce_verifier.to_string()
		});
	}
}

impl DatabaseOperations for Session {
	fn get(id: &str) -> Option<Self> {
		let collection = database::get_collection_sessions();
		database::find_one(id, &collection)
	}

	fn insert(&self) -> Result<(), String> {
		let collection = database::get_collection_sessions();
		database::insert_item(&collection, &self)?;

		Ok(())
	}

	fn update(&self)  -> Result<(), String> {
		let collection = database::get_collection_sessions();
		database::replace_one(&self.id, &self, &collection)
	}

	fn delete(&self)  -> Result<(), String> {
		let collection = database::get_collection_sessions();
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
		assert!(session.user_id.is_none());
	}

	#[test]
	fn set_user() {
		const USER_ID: i64 = 999555i64;

		let mut session = Session::new();
		session.set_user(USER_ID);

		assert!(session.user_id.is_some());
		assert_eq!(session.user_id.unwrap(), USER_ID);
	}

	#[test]
	fn set_oauth_data() {
		const STATE: &str = "aaabbbccc";
		const PKCE_VERIFIER: &str = "abcdef123456";

		let mut session = Session::new();
		session.set_oauth_data(STATE, PKCE_VERIFIER);

		assert!(session.oauth.is_some());

		let oauth = session.oauth.unwrap();
		assert_eq!(oauth.state, STATE);
		assert_eq!(oauth.pkce_verifier, PKCE_VERIFIER);
	}
}