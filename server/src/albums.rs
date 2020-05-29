use serde::{Serialize, Deserialize};

use crate::ids;

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
	pub fn create(title: &str) -> Self {
		let id = ids::create_unique_id();

		Self {
			id,
			title: title.to_string(),
			thumb_photo_id: None,
			photos: vec!{}
		}
	}
}