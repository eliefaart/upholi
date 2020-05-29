use serde::{Serialize, Deserialize};

/*
	I want to get rid of this module,
	will need to move these structs to other places over time.
*/

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientAlbum {
	pub title: Option<String>,
	pub thumb_photo: Option<ClientPhoto>,
	pub photos: Option<Vec<ClientPhoto>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientPhoto {
	pub id: String,
	pub width: i32,
	pub height: i32
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAlbum {
	pub title: Option<String>,
	pub thumb_photo_id: Option<String>,
	pub photos: Option<Vec<String>>
}