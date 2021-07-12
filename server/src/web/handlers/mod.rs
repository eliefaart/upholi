pub mod oauth2;
pub mod photos;
pub mod albums;
pub mod collections;

// Request types
mod requests {
	use serde::Deserialize;

	#[derive(Deserialize)]
	pub struct UploadPhoto {
		/// Encrypted data, contains width, height, exif, etc
		pub data: EncryptedData,
		pub data_version: u8,
		/// Key that all data and file bytes of this photo is encrypted with. This key is encrypted with the owner's private key.
		pub key: EncryptedData,

		pub share_keys: Vec<ShareKey>
	}

	#[derive(Deserialize)]
	pub struct ShareKey {
		id: String,
		key: EncryptedData
	}

	#[derive(Deserialize)]
	pub struct EncryptedData {
		pub nonce: String,
		pub data: String
	}













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
		pub thumb_photo_id: Option<String>,
		pub photos: Option<Vec<String>>,
		pub tags: Option<Vec<String>>
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
	pub struct AuthenticateToCollection {
		pub password: Option<String>,
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
		pub id: String,
		pub title: String,
		pub thumb_photo: Option<PhotoSmall>,
		pub photos: Vec<PhotoSmall>,
		pub tags: Vec<String>
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
			let mut photo_ids: Vec<&str> = Vec::new();

			for id in album.photos.iter() {
				photo_ids.push(&id[..]);
			}

			Self {
				id: album.id,
				title: album.title,
				thumb_photo: {
					if let Some(thumb_photo_id) = album.thumb_photo_id {
						match Photo::get(&thumb_photo_id) {
							Ok(thumb_photo_opt) => thumb_photo_opt.map(PhotoSmall::from),
							Err(_) => None
						}
					} else {
						None
					}
				},
				photos: {
					match Photo::get_with_ids(&photo_ids) {
						Ok(photos) => {
							let mut result_photos = Vec::new();
							for photo in photos {
								result_photos.push(PhotoSmall::from(photo));
							}

							result_photos
						}
						Err(_) => vec!{}
					}
				},
				tags: album.tags
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
				.unwrap_or_else(|_| Vec::new());

			let mut collection_albums: Vec<ClientCollectionAlbum> = Vec::new();
			for album_id in &collection.albums {
				let album = albums.iter().find(|album| &album.id == album_id);
				if let Some(album) = album {
					let client_album = ClientCollectionAlbum {
						id: album.id.to_string(),
						title: album.title.to_string(),
						thumb_photo_id: album.thumb_photo_id.as_ref().map(|thumb_photo_id| thumb_photo_id.to_string())
					};
					collection_albums.push(client_album);
				}
			}

            ClientCollection {
				id: collection.id.to_string(),
				title: collection.title.to_string(),
				albums: collection_albums,
				sharing: ClientCollectionSharingOptions {
					require_password: collection.sharing.password_hash.is_some(),
					token: collection.sharing.token.to_string()
				}
            }
        }
    }
}