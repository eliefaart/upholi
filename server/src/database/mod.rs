use async_trait::async_trait;

use upholi_lib::http::request::{FindEntity, FindSharesFilter};
use upholi_lib::http::response::{Photo, PhotoMinimal};

use self::entities::share::Share;
use self::entities::user::User;
use crate::error::*;

pub mod entities;
mod mongodb;

static COLLECTION_SESSIONS: &str = "sessions";
static COLLECTION_USERS: &str = "users";
static COLLECTION_PHOTOS: &str = "photos";
static COLLECTION_ALBUMS: &str = "albums";
static COLLECTION_SHARES: &str = "shares";

pub struct SortField<'a> {
	pub field: &'a str,
	pub ascending: bool,
}

/// Add standard CRUD operations to an entity
#[async_trait]
pub trait DatabaseEntity {
	/// Get an existing item
	async fn get(id: &str) -> Result<Option<Self>>
	where
		Self: std::marker::Sized;

	/// Insert item as new record
	async fn insert(&self) -> Result<()>;

	/// Store this instance in its current state
	async fn update(&self) -> Result<()>;

	/// Delete this item from database
	async fn delete(&self) -> Result<()>;
}

/// Adds database operations to an entity that targets multiple items
#[async_trait]
pub trait DatabaseEntityBatch {
	/// Get all items with an id contained within given array
	async fn get_many(ids: &[&str]) -> Result<Vec<Self>>
	where
		Self: std::marker::Sized;

	async fn delete_many(ids: &[&str]) -> Result<()>;
}

/// Adds get operations to an entity, but return a minimal/slim version of an entity
#[async_trait]
pub trait DatabaseEntityMinimal {
	type TMinimal;

	/// Get all items with an id contained within given array
	async fn get_minimal(id: &str) -> Result<Option<Self::TMinimal>>
	where
		Self: std::marker::Sized;

	/// Get all items with an id contained within given array
	async fn get_many_minimal(ids: &[&str]) -> Result<Vec<Self::TMinimal>>
	where
		Self: std::marker::Sized;

	async fn get_all_for_user_minimal(user_id: String) -> Result<Vec<Self::TMinimal>>
	where
		Self: std::marker::Sized;
}

/// Add database operations to an entity, which are targetted only to items owned by given user
#[async_trait]
pub trait DatabaseEntityUserOwned: DatabaseEntity {
	async fn get_for_user(id: &str, user_id: String) -> Result<Option<Self>>
	where
		Self: std::marker::Sized;

	async fn get_all_for_user(user_id: String) -> Result<Vec<Self>>
	where
		Self: std::marker::Sized;

	async fn get_many_for_user(ids: &[&str], user_id: String) -> Result<Vec<Self>>
	where
		Self: std::marker::Sized;
}

/// Get a single item from a collection
async fn find_one<T: serde::de::DeserializeOwned>(collection: &str, id: &str, limit_fields: Option<Vec<String>>) -> Result<Option<T>> {
	mongodb::find_one(collection, id, limit_fields).await
}

/// Get multiple items from a collection
async fn find_many<T: serde::de::DeserializeOwned>(
	collection: &str,
	user_id: Option<&str>,
	ids: Option<&[&str]>,
	sort_field: Option<&SortField<'_>>,
	limit_fields: Option<Vec<String>>,
) -> Result<Vec<T>> {
	mongodb::find_many(collection, user_id, ids, sort_field, limit_fields).await
}

/// Insert a single item into a collection.
/// Returns the ID of created item if succesfull.
async fn insert_one<T: serde::Serialize + serde::de::DeserializeOwned>(collection: &str, item: &T) -> Result<String> {
	mongodb::insert_one(collection, item).await
}

/// Replace a single existing item with a new version in its entirety
async fn replace_one<T: serde::Serialize>(collection: &str, id: &str, replacement: &T) -> Result<()> {
	mongodb::replace_one(collection, id, replacement).await
}

/// Delete an item from a collection
async fn delete_one(collection: &str, id: &str) -> Result<()> {
	mongodb::delete_one(collection, id).await
}

/// Delete multiple items from a collection
async fn delete_many(collection: &str, ids: &[&str]) -> Result<()> {
	mongodb::delete_many(collection, ids).await
}

/// Get multiple photos
pub async fn find_photos(photos: Vec<FindEntity>) -> Result<Vec<Photo>> {
	mongodb::find_photos(photos).await
}

/// Get multiple photos
pub async fn find_photos_minimal(photos: Vec<FindEntity>) -> Result<Vec<PhotoMinimal>> {
	mongodb::find_photos_minimal(photos).await
}

/// Check if a photo already exists for user, by hash
pub async fn photo_exists_for_user(user_id: &str, hash: &str) -> Result<bool> {
	mongodb::photo_exists_for_user(user_id, hash).await
}

/// Get user for given ID provider name and user-ID, if it exists
pub async fn get_user_by_username(username: &str) -> Result<Option<User>> {
	mongodb::get_user_by_username(username).await
}

/// Find shares based on certain filters
pub async fn find_shares(user_id: &str, filters: FindSharesFilter) -> Result<Vec<Share>> {
	mongodb::find_shares(user_id, filters).await
}
