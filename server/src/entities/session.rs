use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use crate::database;
use crate::database::{Database, DatabaseEntity};
use crate::ids;
use crate::error::*;

/// A client session
#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
	pub id: String,
	pub user_id: Option<String>,
	created_on: chrono::DateTime<Utc>,

	/// Contains data related to an oauth login attempt
	/// Such as: state id, PKCE tokens.
	/// The value of this field will be None once authorization has completed
	pub oauth: Option<OauthData>
}

/// Contains data related to an oauth login attempt
#[derive(Debug, Serialize, Deserialize)]
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

	pub fn set_user(&mut self, user_id: &str) {
		self.user_id = Some(user_id.to_string());
	}

	pub fn set_oauth_data(&mut self, state: &str, pkce_verifier: &str) {
		self.oauth = Some(OauthData{
			state: state.to_string(),
			pkce_verifier: pkce_verifier.to_string()
		});
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