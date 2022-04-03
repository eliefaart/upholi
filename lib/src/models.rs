use serde::{Deserialize, Serialize};

/// An encrypted photo
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedPhoto {
	pub id: String,
	pub user_id: String,
	/// Hash string of original file bytes
	pub hash: String,
	/// Width of photo
	pub width: i32,
	/// Height of photo
	pub height: i32,
	/// A timestamp of the photo used for sorting purposes.
	/// To be filles with the datetime a photo was taken on, or uploaded on.
	pub timestamp: i64,
	pub data: crate::EncryptedData,
	pub key: crate::EncryptedData,
	pub thumbnail_nonce: String,
	pub preview_nonce: String,
	pub original_nonce: String,
}

/// An encrypted photo model, for insert and update operations
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedPhotoUpsert {
	pub hash: String,
	pub width: u32,
	pub height: u32,
	pub timestamp: i64,
	pub data: crate::EncryptedData,
	pub key: crate::EncryptedData,
	pub key_hash: String,
	pub thumbnail_nonce: String,
	pub preview_nonce: String,
	pub original_nonce: String,
}

/// A very small photo model containing just its dimensions
#[derive(Deserialize, Clone, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PhotoMinimal {
	pub id: String,
	pub width: u32,
	pub height: u32,
}

/// An encrypted album
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedAlbum {
	pub id: String,
	pub user_id: String,
	pub data: crate::EncryptedData,
	pub key: crate::EncryptedData,
}

/// An encrypted album model, for insert and update operations
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedAlbumUpsert {
	pub data: crate::EncryptedData,
	pub key: crate::EncryptedData,
	pub key_hash: String,
}

/// An encrypted share
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedShare {
	pub id: String,
	pub user_id: String,
	pub identifier_hash: String,
	pub type_: crate::ShareType,
	pub password: crate::EncryptedData,
	pub key: crate::EncryptedData,
	pub data: crate::EncryptedData,
}

/// An encrypted share model, for insert and update operations
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedShareUpsert {
	pub identifier_hash: String,
	pub type_: crate::ShareType,
	pub password: crate::EncryptedData,
	pub data: crate::EncryptedData,
	pub key: crate::EncryptedData,
}
