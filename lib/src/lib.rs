use std::fmt;

use serde::{Deserialize, Serialize};

pub mod http;
pub mod ids;
pub mod passwords;
pub mod result;

pub enum PhotoVariant {
	Original,
	Preview,
	Thumbnail,
}

impl PhotoVariant {
	// pub fn to_string(&self) -> String {
	// 	match self {
	// 		PhotoVariant::Thumbnail => "thumbnail".into(),
	// 		PhotoVariant::Preview => "preview".into(),
	// 		PhotoVariant::Original => "original".into(),
	// 	}
	// }
}

impl fmt::Display for PhotoVariant {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			PhotoVariant::Thumbnail => write!(f, "thumbnail"),
			PhotoVariant::Preview => write!(f, "preview"),
			PhotoVariant::Original => write!(f, "original")
		}
	}
}

impl From<PhotoVariant> for String {
	fn from(photo_variant: PhotoVariant) -> String {
		photo_variant.to_string()
	}
}

impl From<&PhotoVariant> for String {
	fn from(photo_variant: &PhotoVariant) -> String {
		photo_variant.to_string()
	}
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedData {
	pub nonce: String,
	pub base64: String,
	/// Version of format of data that was encrypted.
	/// For future use.
	pub format_version: i32,
}

impl Clone for EncryptedData {
	fn clone(&self) -> Self {
		Self {
			base64: self.base64.clone(),
			nonce: self.nonce.clone(),
			format_version: self.format_version,
		}
	}
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ShareType {
	Album,
}

impl Clone for ShareType {
	fn clone(&self) -> Self {
		match *self {
			Self::Album => Self::Album,
		}
	}
}
