use serde::{Deserialize, Serialize};
use upholi_lib::result::Result;

use self::{photo::Photo, share::ShareData};

pub mod album;
pub mod photo;
pub mod share;

pub trait Entity {
	type TEncrypted;
	type TData;
	type TJavaScript;

	/// Init from encrypted data using the entity's key.
	fn from_encrypted(source: Self::TEncrypted, key: &[u8]) -> Result<Self>
	where
		Self: std::marker::Sized;
	/// Init from encrypted data using the entity's owner's private key.
	fn from_encrypted_with_owner_key(source: Self::TEncrypted, key: &[u8]) -> Result<Self>
	where
		Self: std::marker::Sized;
	fn get_key(&self) -> &[u8];
	fn get_id(&self) -> &str;
	fn get_data(&self) -> &Self::TData;
	fn get_data_mut(&mut self) -> &mut Self::TData;
	fn as_js_value(&self) -> &Self::TJavaScript;
}

pub trait Shareable {
	fn create_share_data(&self, key: &[u8], photos: &[Photo]) -> Result<ShareData>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EntityKey {
	pub id: String,
	pub key: String,
}
