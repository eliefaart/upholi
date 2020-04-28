use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Photo {
	#[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing 
    pub id: bson::oid::ObjectId,
	pub name: String,
	pub width: u32,
	pub height: u32,
	pub path_thumbnail: String,
	pub path_preview: String,
	pub path_original: String
}

#[derive(Serialize, Deserialize)]
pub struct BsonPhoto {
	#[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing 
    pub id: bson::oid::ObjectId,
	pub name: String,
	pub width: u32,
	pub height: u32,
	pub path_thumbnail: String,
	pub path_preview: String,
	pub path_original: String
}