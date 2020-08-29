pub mod oauth2;
pub mod photos;
pub mod albums;
pub mod collections;

// Request types
mod requests {
	use serde::Deserialize;

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct OauthCallback {
		pub code: String,
		pub state: String
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct CreateAlbum {
		pub title: String
	}

	#[derive(Deserialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UpdateAlbum {
		pub title: Option<String>,
		pub public: Option<bool>,
		pub thumb_photo_id: Option<String>,
		pub photos: Option<Vec<String>>
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct CreateCollection {
		pub title: String
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct UpdateCollection {
		pub title: Option<String>,
		pub public: Option<bool>,
		pub albums: Option<Vec<String>>,
		pub sharing: Option<UpdateCollectionSharingOptions>
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct UpdateCollectionSharingOptions {
		pub shared: bool,
		pub require_password: bool,
		pub password: Option<String>
	}
}

// Response types
mod responses {
	use serde::Serialize;
	use crate::entities::photo::Photo;
	use crate::entities::album::Album;
	use crate::entities::collection::Collection;
	use crate::database::{DatabaseEntity, DatabaseEntityBatch};
 
	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct PhotoSmall {
		id: String,
		width: u16,
		height: u16
	}

	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct ClientAlbum {
		pub title: String,
		pub public: bool,
		pub thumb_photo: Option<PhotoSmall>,
		pub photos: Vec<PhotoSmall>
	}
	
	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct ClientCollectionAlbum {
		pub id: String,
		pub title: String,
		pub thumb_photo_id: Option<String>
	}

	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct ClientCollection {
		pub id: String,
		pub title: String,
		pub albums: Vec<ClientCollectionAlbum>,
		pub sharing: ClientCollectionSharingOptions
	}

	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct ClientCollectionSharingOptions {
		pub shared: bool,
		pub require_password: bool,
		pub token: String
	}
    
	impl From<Photo> for PhotoSmall {
		fn from(photo: Photo) -> Self {
			Self {
				id: photo.id,
				width: photo.width as u16,
				height: photo.height as u16
			}
		}
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
							Ok(thumb_photo_opt) => {
								match thumb_photo_opt {
									Some(thumb_photo) => Some(PhotoSmall::from(thumb_photo)),
									None => None
								}
							},
							Err(_) => None
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
								result_photos.push(PhotoSmall::from(photo));
							}

							result_photos
						}
						Err(_) => vec!{}
					}
				}
			}
		}
    }
    
	impl From<&Collection> for ClientCollection {
        fn from(collection: &Collection) -> Self {
			let mut album_ids: Vec<&str> = Vec::new();
			for album in &collection.albums {
				album_ids.push(album);
			}

			let albums = Album::get_with_ids(&album_ids)
				.unwrap_or_else(|_| vec!{});

			let collection_albums = albums.iter().map(|album| ClientCollectionAlbum {
				id: album.id.to_string(),
				title: album.title.to_string(),
				thumb_photo_id: { 
					match &album.thumb_photo_id {
						Some(thumb_photo_id) => Some(thumb_photo_id.to_string()),
						None => None
					}
				}
			}).collect();

            ClientCollection {
				id: collection.id.to_string(),
				title: collection.title.to_string(),
				albums: collection_albums,
				sharing: ClientCollectionSharingOptions {
					shared: collection.sharing.shared,
					require_password: collection.sharing.password_hash.is_some(),
					token: collection.sharing.token.to_string()
				}
            }
        }
    }
}