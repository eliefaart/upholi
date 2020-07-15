use serde::{Serialize, Deserialize};
use crate::albums::Album;
use crate::photos::Photo;
use crate::database::{DatabaseOperations, DatabaseBatchOperations};
/*
	I want to get rid of this module,
	will need to move these structs to other places over time.
*/

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientAlbum {
	pub title: String,
	pub public: bool,
	pub thumb_photo: Option<ClientPhoto>,
	pub photos: Vec<ClientPhoto>
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
	pub public: Option<bool>,
	pub thumb_photo_id: Option<String>,
	pub photos: Option<Vec<String>>
}

impl From<Album> for ClientAlbum {
	fn from(album: Album) -> Self {
		let mut ids: Vec<&str> = Vec::new();
		
		for id in album.photos.iter() {
			ids.push(&id[..]);
		}

		Self {
			title: album.title,
			public: album.public,
			thumb_photo: {
				if let Some(thumb_photo_id) = album.thumb_photo_id {
					match Photo::get(&thumb_photo_id) {
						Some(thumb_photo) => Some(thumb_photo.to_client_photo()),
						None => None
					}
				} else {
					None
				}
			},
			photos: {
				match Photo::get_with_ids(&ids) {
					Ok(photos) => {
						let mut result_photos = Vec::new();
						for photo in photos {
							result_photos.push(photo.to_client_photo());
						}

						result_photos
					}
					Err(_) => vec!{}
				}
			}
		}
	}
}