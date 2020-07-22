use crate::error::*;

pub mod mongo;

pub const COLLECTION_SESSIONS: &str = "sessions";
pub const COLLECTION_PHOTOS: &str = "photos";
pub const COLLECTION_ALBUMS: &str = "albums";

// Get the implementation of the database traits
pub fn get_database() -> impl Database + DatabaseExt {
	mongo::MongoDatabase{}
}

pub struct SortField<'a> {
	pub field: &'a str,
	pub ascending: bool,
}

/// General CRUD functions for a database implementation
pub trait Database {
	/// Get a single item from a collection
	fn find_one<'de, T: serde::Deserialize<'de>>(&self, collection: &str, id: &str) 
		-> Result<Option<T>>;

	/// Get multiple items from a collection
	fn find_many<'de, T: serde::Deserialize<'de>>(&self, collection: &str, user_id: Option<i64>, ids: Option<&[&str]>, sort_field: Option<&SortField>) 
		-> Result<Vec<T>>;

	/// Insert a single item into a collection.
	/// Returns the ID of created item if succesfull.
	fn insert_one<T: serde::Serialize>(&self, collection: &str, item: &T) 
		-> Result<String>;

	/// Replace a single existing item with a new version in its entirety
	fn replace_one<T: serde::Serialize>(&self, collection: &str, id: &str, replacement: &T) 
		-> Result<()>;

	/// Delete an item from a collection
	fn delete_one(&self, collection: &str, id: &str) 
		-> Result<()>;

	/// Delete multiple items from a collection
	fn delete_many(&self, collection: &str, ids: &[&str]) 
		-> Result<()>;
}

/// Specific database actions that this application needs
/// TODO: Give this a better name
pub trait DatabaseExt : Database {
	/// Remove photos with given photo_ids from all albums containing any of these photos
	fn remove_photos_from_all_albums(&self, photo_ids: &[&str]) -> Result<()>;

	/// Unset thumbnail of all album where thumbnail is set to any of given photo_ids
	fn remove_thumbs_from_all_albums(&self, photo_ids: &[&str]) -> Result<()>;

	/// Check if a photo already exists for user, by hash
	fn photo_exists_for_user(&self, user_id: i64, hash: &str) -> Result<bool>;
}

/// Add standard CRUD operations to a struct
pub trait DatabaseEntity {
	/// Get an existing item
	fn get(id: &str) -> Option<Self>
		where Self: std::marker::Sized;

	/// Insert item as new record
	fn insert(&self) -> Result<()>;

	/// Store this instance in its current state
	fn update(&self) -> Result<()>;

	/// Delete this item from database
	fn delete(&self) -> Result<()>;
}

/// Adds CRUD operations to a struct that targets multiple items
pub trait DatabaseEntityBatch {
	/// Get all items with an id contained within given array
	/// TODO: Merge with DatabaseEntity?
	fn get_with_ids(ids: &[&str]) -> Result<Vec<Self>>
		where Self: std::marker::Sized;
}

/// Add database operations to a struct, which are targetted only to entries owned by given user
pub trait DatabaseUserEntity: DatabaseEntity {
	fn get_as_user(id: &str, user_id: i64) -> Result<Option<Self>>
		where Self: std::marker::Sized;

	fn get_all_as_user(user_id: i64) -> Result<Vec<Self>>
		where Self: std::marker::Sized;

	fn get_all_with_ids_as_user(ids: &[&str], user_id: i64) -> Result<Vec<Self>>
		where Self: std::marker::Sized;
}