use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedPhoto {
	pub hash: String,
	pub width: u32,
	pub height: u32,
	pub data: crate::EncryptedData,
	pub key: crate::EncryptedData,
	pub key_hash: String,
	pub thumbnail_nonce: String,
	pub preview_nonce: String,
	pub original_nonce: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedAlbum {
	pub data: crate::EncryptedData,
	pub key: crate::EncryptedData,
	pub key_hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedShare {
	pub identifier_hash: String,
	pub type_: crate::ShareType,
	pub password: crate::EncryptedData,
	pub data: crate::EncryptedData,
	pub key: crate::EncryptedData,
}
