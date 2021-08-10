use serde::{Deserialize,Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	/// Hash of original photo file
	pub hash: String,
	pub width: u32,
	pub height: u32,
	pub content_type: String,
	pub exif: crate::Exif
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	pub name: String,
	pub tags: Vec<String>,
	pub photos: Vec<String>
}