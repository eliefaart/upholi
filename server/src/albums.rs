use serde::{Serialize, Deserialize};

use crate::types;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	#[serde(default)] 
	pub id: String,
	pub title: String,
	#[serde(default)]
	pub thumb_photo_id: Option<String>,
	#[serde(default)]
	pub photos: Vec<String>
}

impl Album {
	pub fn to_bson_album(&self) -> types::BsonAlbum {
		let mut photos: Vec<bson::oid::ObjectId> = Vec::new();
		for photo_id in &self.photos {
			if photo_id != "" {
				photos.push(types::string_to_object_id(&photo_id).unwrap());
			}
		}

		types::BsonAlbum{
			id: types::string_to_object_id_or_new(&self.id),
			title: self.title.to_string(),
			thumb_photo_id: match &self.thumb_photo_id { Some(id) => Some(types::string_to_object_id(id).unwrap()), None => None },
			photos: photos
		}
	}
}