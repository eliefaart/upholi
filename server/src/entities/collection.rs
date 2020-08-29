use serde::{Serialize, Deserialize};

use crate::database;
use crate::database::{Database, DatabaseEntity, DatabaseUserEntity};
use crate::ids;
use crate::error::*;
use crate::entities::AccessControl;
use crate::entities::user::User;


/// A Collection is a collection of 0..n albums 
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
	pub id: String,
	pub user_id: String,
	pub title: String,
	pub public: bool,
	pub albums: Vec<String>
}

impl Collection {
	pub fn new(user_id: &str, title: &str) -> Self {
		Self {
			id: ids::create_unique_id(),
			user_id: user_id.to_string(),
			title: title.to_string(),
			public: false,
			albums: vec!{}
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
	fn user_has_access(&self, user_opt: Option<User>) -> bool {
		if let Some(user) = user_opt {
			self.user_id == user.id || self.public
		} 
		else {
			self.public
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ids::create_unique_id;

	#[test]
	fn access_private() {
		let user_collection_owner = create_dummy_user();
		let user_not_collection_owner = create_dummy_user();

		let mut collection = create_dummy_collection_with_id("");
		collection.public = false;
		collection.user_id = user_collection_owner.id.to_string();

		// Only the user that owns the collection may access it
		assert_eq!(collection.user_has_access(Some(user_collection_owner)), true);
		assert_eq!(collection.user_has_access(Some(user_not_collection_owner)), false);
		assert_eq!(collection.user_has_access(None), false);
	}

	#[test]
	fn access_public() {
		let user_collection_owner = create_dummy_user();
		let user_not_collection_owner = create_dummy_user();

		let mut collection = create_dummy_collection_with_id("");
		collection.public = true;
		collection.user_id = user_collection_owner.id.to_string();

		// Everyone may access the collection, because it is public
		assert_eq!(collection.user_has_access(Some(user_collection_owner)), true);
		assert_eq!(collection.user_has_access(Some(user_not_collection_owner)), true);
		assert_eq!(collection.user_has_access(None), true);
	}

	fn create_dummy_collection_with_id(id: &str) -> Collection {
		Collection{
			id: id.to_string(),
			user_id: create_unique_id(),
			public: false,
			title: "title".to_string(),
			albums: vec!{}
		}
	}

	fn create_dummy_user() -> User {
		User{
			id: create_unique_id(), 
			identity_provider: "".to_string(), 
			identity_provider_user_id: "".to_string()
		}
	}
}