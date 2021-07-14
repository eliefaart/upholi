use upholi_lib::http::response::PhotoMinimal;

use crate::error::*;
use crate::entities::album::Album;
use crate::entities::collection::Collection;
use crate::entities::user::User;

mod mongodb;

pub const COLLECTION_SESSIONS: &str = "sessions";
pub const COLLECTION_USERS: &str = "users";
pub const COLLECTION_PHOTOS: &str = "photos";
pub const COLLECTION_PHOTOS_NEW: &str = "photos2";
pub const COLLECTION_ALBUMS: &str = "albums";
pub const COLLECTION_COLLECTIONS: &str = "collections";

// Get the implementation of the database traits
pub fn get_database() -> impl Database + DatabaseExt {
	mongodb::MongoDatabase::new()
}

pub struct SortField<'a> {
	pub field: &'a str,
	pub ascending: bool,
}

/// General CRUD functions for a database implementation
pub trait Database {
	/// Get a single item from a collection
	fn find_one<T: serde::de::DeserializeOwned>(&self, collection: &str, id: &str)
		-> Result<Option<T>>;

	/// Get multiple items from a collection
	fn find_many<T: serde::de::DeserializeOwned>(&self, collection: &str, user_id: Option<&str>, ids: Option<&[&str]>, sort_field: Option<&SortField>)
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
	/// Get all photos of given user, returning only minimal info per user.
	fn get_photos_for_user(&self, user_id: &str) -> Result<Vec<PhotoMinimal>>;

	/// Remove photos with given photo_ids from all albums containing any of these photos
	fn remove_photos_from_all_albums(&self, photo_ids: &[&str]) -> Result<()>;

	/// Unset thumbnail of all album where thumbnail is set to any of given photo_ids
	fn remove_thumbs_from_all_albums(&self, photo_ids: &[&str]) -> Result<()>;

	/// Check if a photo already exists for user, by hash
	fn photo_exists_for_user(&self, user_id: &str, hash: &str) -> Result<bool>;

	/// Get all albums that contain given photo
	fn get_albums_with_photo(&self, photo_id: &str) -> Result<Vec<Album>>;

	/// Get user for given ID provider name and user-ID, if it exists
	fn get_user_for_identity_provider(&self, identity_provider: &str, identity_provider_user_id: &str) -> Result<Option<User>>;

	/// Get a single collection by its share token.
	/// Note: this does not check if collection is shared at all.
	fn get_collection_by_share_token(&self, token: &str) -> Result<Option<Collection>>;

	/// Get all collections that contain given album
	fn get_collections_with_album(&self, album_id: &str) -> Result<Vec<Collection>>;
}

/// Add standard CRUD operations to a struct
pub trait DatabaseEntity {
	/// Get an existing item
	fn get(id: &str) -> Result<Option<Self>>
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
pub trait DatabaseUserEntity : DatabaseEntity {
	fn get_as_user(id: &str, user_id: String) -> Result<Option<Self>>
		where Self: std::marker::Sized;

	fn get_all_as_user(user_id: String) -> Result<Vec<Self>>
		where Self: std::marker::Sized;

	fn get_all_with_ids_as_user(ids: &[&str], user_id: String) -> Result<Vec<Self>>
		where Self: std::marker::Sized;
}