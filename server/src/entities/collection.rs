use crate::{entities::Session, passwords::verify_password_hash};
use serde::{Serialize, Deserialize};

use crate::database;
use crate::database::{Database, DatabaseExt, DatabaseEntity, DatabaseUserEntity};
use crate::ids;
use crate::error::*;
use crate::entities::AccessControl;

/// A Collection is a container of 0..n albums. It is publicly accessible, but may be protected by a password.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
	pub id: String,
	pub user_id: String,
	pub title: String,
	pub albums: Vec<String>,
	pub sharing: CollectionSharingOptions
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionSharingOptions {
	pub token: String,
	pub password_hash: Option<String>
}

impl Collection {
	pub fn new(user_id: &str, title: &str) -> Self {
		Self {
			id: ids::create_unique_id(),
			user_id: user_id.to_string(),
			title: title.to_string(),
			albums: vec!{},
			sharing: CollectionSharingOptions {
				token: ids::create_unique_id(),
				password_hash: Some(String::new()),
			}
		}
	}

	pub fn get_by_share_token(token: &str) -> Result<Option<Self>> {
		match database::get_database().get_collection_by_share_token(token)? {
			Some(collection) => {
				Ok(Some(collection))
			},
			None => Ok(None)
		}
	}

	/// Rotate the token with which a collection may be accessed by clients other than the user that owns a collection
	pub fn rotate_share_token(&mut self) {
		self.sharing.token = crate::ids::create_unique_id();
	}

	/// Verify if given (unhashed) password matches the password set in the collection.
	/// Returns false if collection has no password.
	pub fn password_correct(&self, password: &str) -> bool {
		match &self.sharing.password_hash {
			Some(phc_hash) => verify_password_hash(&password, &phc_hash),
			None => false
		}
	}
}

impl DatabaseEntity for Collection {
	fn get(id: &str) -> Result<Option<Self>> {
		database::get_database().find_one(database::COLLECTION_COLLECTIONS, id)
	}

	fn insert(&self) -> Result<()> {
		database::get_database().insert_one(database::COLLECTION_COLLECTIONS, &self)?;
		Ok(())
	}

	fn update(&self)  -> Result<()> {
		database::get_database().replace_one(database::COLLECTION_COLLECTIONS, &self.id, self)
	}

	fn delete(&self)  -> Result<()> {
		database::get_database().delete_one(database::COLLECTION_COLLECTIONS, &self.id)
	}
}

impl DatabaseUserEntity for Collection {
	fn get_as_user(id: &str, user_id: String) -> Result<Option<Self>>{
		match Self::get(id)? {
			Some(collection) => {
				if collection.user_id != user_id {
					Err(Box::from(format!("User {} does not have access to collection {}", user_id, collection.id)))
				} else {
					Ok(Some(collection))
				}
			}
			None => Ok(None)
		}
	}

	fn get_all_as_user(user_id: String) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_COLLECTIONS, Some(&user_id), None, None)
	}

	fn get_all_with_ids_as_user(ids: &[&str], user_id: String) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_COLLECTIONS, Some(&user_id), Some(ids), None)
	}
}

impl AccessControl for Collection {
	fn can_view(&self, session_opt: &Option<Session>) -> bool {
		// If there is no password on collection, everyone may view it.
		// Otherwise user must own the collection, or be authenticated to it.

		match &self.sharing.password_hash {
			Some(_) => {
				match session_opt {
					Some(session) => {
						if let Some(user_id) = &session.user_id {
							if user_id == &self.user_id {
								// Session user owns the collection
								return true;
							}
						}

						session.authenticated_for_collection_tokens.contains(&self.sharing.token)
					},
					None => false
				}
			}
			None => true
		}
	}

    fn can_update(&self, session_opt: &Option<Session>) -> bool {
		match session_opt {
			Some(session) => {
				match &session.user_id {
					Some(user_id) => &self.user_id == user_id,
					None => false
				}
			}
			None => false
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ids::create_unique_id;

	#[test]
	fn view_collection_without_password() {
		let session_owner = create_dummy_session(true);
		let session_not_owner = create_dummy_session(true);
		let session_anonymous = create_dummy_session(false);

		let mut collection = create_dummy_collection_with_id("");
		collection.user_id = session_owner.user_id.to_owned().unwrap();
		collection.sharing.password_hash = None;

		assert_eq!(collection.can_view(&Some(session_owner)), true);
		assert_eq!(collection.can_view(&Some(session_not_owner)), true);
		assert_eq!(collection.can_view(&Some(session_anonymous)), true);
		assert_eq!(collection.can_view(&None), true);
	}

	#[test]
	fn update_collection_without_password() {
		let session_owner = create_dummy_session(true);
		let session_not_owner = create_dummy_session(true);
		let session_anonymous = create_dummy_session(false);

		let mut collection = create_dummy_collection_with_id("");
		collection.user_id = session_owner.user_id.to_owned().unwrap();
		collection.sharing.password_hash = None;

		assert_eq!(collection.can_update(&Some(session_owner)), true);
		assert_eq!(collection.can_update(&Some(session_not_owner)), false);
		assert_eq!(collection.can_update(&Some(session_anonymous)), false);
		assert_eq!(collection.can_update(&None), false);
	}

	#[test]
	fn view_collection_with_password_provide_correct_password() {
		// TODO
	}

	#[test]
	fn view_collection_with_password_provide_incorrect_password() {
		// TODO
	}

	fn create_dummy_collection_with_id(id: &str) -> Collection {
		Collection::new(id, &create_unique_id())
	}

	fn create_dummy_session(with_user: bool) -> Session {
		let mut session = Session::new();

		if with_user {
			session.user_id = Some(create_unique_id());
		}

		session
	}
}