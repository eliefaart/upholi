use crate::database::DatabaseEntity;
use crate::error::*;
use async_trait::async_trait;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use upholi_lib::ids::create_unique_id;

/// A client session
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
	pub id: String,
	pub user_id: Option<String>,
	created_on: chrono::DateTime<Utc>,
}

impl Session {
	pub fn new() -> Self {
		Self {
			id: create_unique_id(),
			user_id: None,
			created_on: Utc::now(),
		}
	}

	pub fn set_user(&mut self, user_id: &str) {
		self.user_id = Some(user_id.to_string());
	}
}

#[async_trait]
impl DatabaseEntity for Session {
	/// Get an existing item
	async fn get(id: &str) -> Result<Option<Self>> {
		super::super::find_one(super::super::COLLECTION_SESSIONS, id, None).await
	}

	/// Insert item as new record
	async fn insert(&self) -> Result<()> {
		super::super::insert_one(super::super::COLLECTION_SESSIONS, self).await?;
		Ok(())
	}

	/// Store this instance in its current state
	async fn update(&self) -> Result<()> {
		super::super::replace_one(super::super::COLLECTION_SESSIONS, &self.id, self).await
	}

	/// Delete this item from database
	async fn delete(&self) -> Result<()> {
		super::super::delete_one(super::super::COLLECTION_SESSIONS, &self.id).await
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