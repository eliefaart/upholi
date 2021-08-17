use serde::{Deserialize,Serialize};

// Note:
// The 'id' fields are only used when sending the data to javascript.
// They will be empty when encrypted before sending to server.
// I would have to make identical structs (+ id field) for these types to keep things pure.
// I don't want that yet because I already do a lot of data cloning. Leaving for later.

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	pub id: String,
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
	pub id: String,
	pub title: String,
	pub tags: Vec<String>,
	pub photos: Vec<String>,
	pub thumbnail_photo_id: Option<String>,
}