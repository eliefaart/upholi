use upholi_lib::http::request::{FindSharesFilter, RequestedEntity};
use upholi_lib::http::response::PhotoMinimal;
use async_trait::async_trait;

use crate::error::*;
use self::entities::share::Share;
use self::entities::user::User;

pub mod entities;
mod mongodb;

const COLLECTION_SESSIONS: &str = "sessions";
const COLLECTION_USERS: &str = "users";
const COLLECTION_PHOTOS: &str = "photos";
const COLLECTION_ALBUMS: &str = "albums";
const COLLECTION_SHARES: &str = "shares";

pub struct SortField<'a> {
	pub field: &'a str,
	pub ascending: bool,
}

/// Add standard CRUD operations to a struct
#[async_trait]
pub trait DatabaseEntity {
	/// Get an existing item
	async fn get(id: &str) -> Result<Option<Self>>
		where Self: std::marker::Sized;

	/// Insert item as new record
	async fn insert(&self) -> Result<()>;

	/// Store this instance in its current state
	async fn update(&self) -> Result<()>;

	/// Delete this item from database
	async fn delete(&self) -> Result<()>;
}

/// Adds CRUD operations to a struct that targets multiple items
#[async_trait]
pub trait DatabaseEntityBatch {
	/// Get all items with an id contained within given array
	/// TODO: Merge with DatabaseEntity?
	async fn get_with_ids(ids: &[&str]) -> Result<Vec<Self>>
		where Self: std::marker::Sized;
}

/// Add database operations to a struct, which are targetted only to entries owned by given user
#[async_trait]
pub trait DatabaseUserEntity : DatabaseEntity {
	async fn get_as_user(id: &str, user_id: String) -> Result<Option<Self>>
		where Self: std::marker::Sized;

	async fn get_all_as_user(user_id: String) -> Result<Vec<Self>>
		where Self: std::marker::Sized;

	async fn get_all_with_ids_as_user(ids: &[&str], user_id: String) -> Result<Vec<Self>>
		where Self: std::marker::Sized;
}

/// Get a single item from a collection
async fn find_one<T: serde::de::DeserializeOwned>(collection: &str, id: &str) -> Result<Option<T>> {
	mongodb::find_one(collection, id).await
}

/// Get multiple items from a collection
async fn find_many<'a, T: serde::de::DeserializeOwned>(collection: &str, user_id: Option<&str>, ids: Option<&[&str]>, sort_field: Option<&SortField<'a>>) -> Result<Vec<T>> {
	mongodb::find_many(collection, user_id, ids, sort_field).await
}

/// Insert a single item into a collection.
/// Returns the ID of created item if succesfull.
async fn insert_one<'de, T: serde::Serialize + serde::Deserialize<'de>>(collection: &str, item: &T) -> Result<String> {
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

/// Get all photos of given user, returning only minimal info per user.
pub async fn get_photos_for_user(user_id: &str) -> Result<Vec<PhotoMinimal>> {
	mongodb::get_photos_for_user(user_id).await
}

/// Get multiple photos
pub async fn get_photos(photos: Vec<RequestedEntity>) -> Result<Vec<PhotoMinimal>> {
	mongodb::get_photos(photos).await
}

/// Delete multiple photos
pub async fn delete_photos(photo_ids: &[&str]) -> Result<()> {
	delete_many(COLLECTION_PHOTOS, photo_ids).await
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
