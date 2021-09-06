use serde::{Deserialize, Serialize};

pub mod result;
pub mod http;
pub mod ids;
pub mod passwords;

pub enum PhotoVariant {
	Original,
	Preview,
	Thumbnail
}

impl PhotoVariant {
	pub fn to_string(&self) -> String {
		match self {
			PhotoVariant::Thumbnail => "thumbnail".into(),
			PhotoVariant::Preview => "preview".into(),
			PhotoVariant::Original => "original".into()
		}
	}
}

impl Into<String> for PhotoVariant {
	fn into(self) -> String {
		self.to_string()
	}
}

impl Into<String> for &PhotoVariant {
	fn into(self) -> String {
		self.to_string()
	}
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedShareKey {
	id: String,
	key: EncryptedData
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShareKey {
	pub id: String,
	pub key: String
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedData {
	pub nonce: String,
	pub base64: String,
	/// Version of format of data that was encrypted.
	/// For future use.
	pub format_version: i32
}

impl Clone for EncryptedData {
	fn clone(&self) -> Self {
		Self {
			base64: self.base64.clone(),
			nonce: self.nonce.clone(),
			format_version: self.format_version
		}
	}
}