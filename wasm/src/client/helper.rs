use super::http::HttpClient;
use crate::entities::album::{self, Album, JsAlbumFull, JsAlbumPhoto};
use crate::entities::photo::{Photo, PhotoData};
use crate::entities::share::{Share, ShareData};
use crate::entities::{Entity, Shareable};
use crate::exif::Exif;
use crate::hashing::compute_sha256_hash;
use crate::images::Image;
use crate::{encryption, hashing};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::RwLock;
use upholi_lib::http::request::{FindEntity, FindSharesFilter, Login, Register};
use upholi_lib::http::response::UserInfo;
use upholi_lib::result::Result;
use upholi_lib::{http::*, PhotoVariant, ShareType};

lazy_static! {
	pub static ref CLIENT: RwLock<UpholiClientHelper> = {
		let window = web_sys::window().expect("Could not find global 'window'.");
		let location = window.location();
		let origin = location.origin().expect("could not determine 'origin'.");

		let client = UpholiClientHelper::new(&origin);
		RwLock::new(client)
	};
}

/// Wrapper struct containing info about bytes to upload.
pub struct PhotoUploadInfo {
	image: Image,
	exif: Option<Exif>,
}

impl PhotoUploadInfo {
	/// Try to construct an object from image file bytes
	pub fn try_from_slice(bytes: &[u8]) -> Result<Self> {
		let exif = Exif::parse_from_photo_bytes(bytes)?;
		let exif_orientation = match &exif {
			Some(exif) => exif.orientation.unwrap_or(1),
			None => 1,
		};

		let image = Image::from_buffer(bytes, exif_orientation as u8)?;
		Ok(Self { image, exif })
	}

	pub fn bytes_original(&self) -> &[u8] {
		&self.image.bytes_original
	}

	pub fn bytes_preview(&self) -> &[u8] {
		&self.image.bytes_preview
	}

	pub fn bytes_thumbnail(&self) -> &[u8] {
		&self.image.bytes_thumbnail
	}
}

/// Helper functions for UpholiClient.
/// This object is not exposed outside the wasm.
pub struct UpholiClientHelper {
	http_client: HttpClient,
}

impl UpholiClientHelper {
	pub fn new(base_url: &str) -> Self {
		Self {
			http_client: HttpClient::new(base_url),
		}
	}

	pub async fn register(&self, username: &str, password: &str) -> Result<()> {
		let password_derived_key = Self::get_key_from_user_credentials(username, password)?;

		// This will be the master encryption key of the user.
		// We encrypt it using the key derived from the user's password,
		// and the encrypted master key is stored server-side.
		let master_key = encryption::symmetric::generate_key();
		let key_encrypted = encryption::symmetric::encrypt_slice(&password_derived_key, &master_key)?;

		let body = Register {
			username: username.into(),
			password: password.into(),
			key: key_encrypted.into(),
		};

		self.http_client.register_user(&body).await
	}

	/// Returns the user's master encryption key when login was succesful
	pub async fn login(&self, username: &str, password: &str) -> Result<Vec<u8>> {
		// derive public/private key pair from password
		// encrypt username with private key
		// send encrypted username to server
		// server will verify by decrypting it using public key

		let body = Login {
			username: username.into(),
			password: password.into(),
		};

		let user = self.http_client.login(&body).await?;

		let password_derived_key = Self::get_key_from_user_credentials(username, password)?;
		let key = encryption::symmetric::decrypt_data_base64(&password_derived_key, &user.key)?;

		Ok(key)
	}

	/// Derive a symmetric encryption key from a user's credentials
	fn get_key_from_user_credentials(username: &str, password: &str) -> Result<Vec<u8>> {
		if username.is_empty() {
			Err(Box::from("Username is empty"))
		} else if password.is_empty() {
			Err(Box::from("Password is empty"))
		} else {
			// The salt is based on username; hash username to ensure minimum length.
			let salt = hashing::compute_sha256_hash(username.as_bytes())?;
			let password_derived_key = encryption::symmetric::derive_key_from_string(password, &salt)?;
			Ok(password_derived_key)
		}
	}

	pub async fn get_user_info(&self) -> Result<UserInfo> {
		self.http_client.get_user_info().await
	}

	pub async fn get_photos(&self) -> Result<Vec<response::PhotoMinimal>> {
		self.http_client.get_photos().await
	}

	pub async fn upload_photo(&self, private_key: &[u8], upload_info: &PhotoUploadInfo) -> Result<String> {
		let mut request_data = Self::get_upload_photo_request_data(upload_info, private_key)?;

		let exists = self.http_client.photo_exists(&request_data.hash).await?;
		if exists {
			// No error, just skipping upload.
			Ok(String::new())
		} else {
			// Decrypt photo key
			let photo_key = crate::encryption::symmetric::decrypt_data_base64(private_key, &request_data.key)?;

			// Encrypt photo bytes
			let thumbnail_encrypted = crate::encryption::symmetric::encrypt_slice(&photo_key, upload_info.bytes_thumbnail())?;
			let preview_encrypted = crate::encryption::symmetric::encrypt_slice(&photo_key, upload_info.bytes_preview())?;
			let original_encrypted = crate::encryption::symmetric::encrypt_slice(&photo_key, upload_info.bytes_original())?;

			// Store nonces in request data
			request_data.thumbnail_nonce = thumbnail_encrypted.nonce;
			request_data.preview_nonce = preview_encrypted.nonce;
			request_data.original_nonce = original_encrypted.nonce;

			self.http_client
				.create_photo(
					&request_data,
					&thumbnail_encrypted.bytes,
					&preview_encrypted.bytes,
					&original_encrypted.bytes,
				)
				.await
		}
	}

	pub async fn get_photo(&self, private_key: &[u8], id: &str, key: &Option<String>) -> Result<Photo> {
		let photo = self.http_client.get_photo(id, key).await?;

		let photo = match key {
			Some(photo_key) => Photo::from_encrypted(photo, &base64::decode_config(photo_key, base64::STANDARD)?)?,
			None => Photo::from_encrypted_with_owner_key(photo, private_key)?,
		};
		Ok(photo)
	}

	pub async fn delete_photos(&self, private_key: &[u8], ids: &[String]) -> Result<()> {
		// Remove photos from all albums they are part of
		let albums = self.get_albums(private_key).await?;
		for album in albums {
			let album_data = album.get_data();
			if album_data.photos.iter().any(|photo| ids.contains(photo)) {
				self.remove_photos_from_album(private_key, album.get_id(), ids).await?;
			}
		}

		// Delete photos
		for id in ids {
			self.http_client.delete_photo(id).await?;
		}

		Ok(())
	}

	pub async fn get_photo_base64(
		&self,
		private_key: &[u8],
		id: &str,
		photo_variant: PhotoVariant,
		key: &Option<String>,
	) -> Result<String> {
		// Get photo bytes
		let encrypted_bytes = self.http_client.get_photo_base64(id, &photo_variant, key).await?;

		// Decrypt photo bytes
		let photo = self.http_client.get_photo(id, key).await?;
		let photo_key = match key {
			Some(photo_key) => base64::decode_config(&photo_key, base64::STANDARD)?,
			None => crate::encryption::symmetric::decrypt_data_base64(private_key, &photo.key)?,
		};
		let nonce = match photo_variant {
			PhotoVariant::Thumbnail => photo.thumbnail_nonce.as_bytes(),
			PhotoVariant::Preview => photo.preview_nonce.as_bytes(),
			PhotoVariant::Original => photo.original_nonce.as_bytes(),
		};
		let bytes = crate::encryption::symmetric::decrypt_slice(&photo_key, nonce, &encrypted_bytes)?;

		Ok(base64::encode_config(&bytes, base64::STANDARD))
	}

	pub async fn get_photo_image_src(
		&self,
		private_key: &[u8],
		id: &str,
		photo_variant: PhotoVariant,
		key: &Option<String>,
	) -> Result<String> {
		if id.is_empty() {
			Ok(String::new())
		} else {
			let photo = self.get_photo(private_key, id, key).await?;
			let photo_data = photo.get_data();
			let base64 = self.get_photo_base64(private_key, id, photo_variant, key).await?;

			let src = format!("data:{};base64,{}", photo_data.content_type, base64);
			Ok(src)
		}
	}

	/// Get data about photo to send as part of the HTTP request's body
	pub fn get_upload_photo_request_data(photo: &PhotoUploadInfo, private_key: &[u8]) -> Result<request::UploadPhoto> {
		// Generate a key and encrypt it
		let photo_key = crate::encryption::symmetric::generate_key();
		let photo_key_hash = compute_sha256_hash(&photo_key)?;
		let photo_key_encrypt_result = crate::encryption::symmetric::encrypt_slice(private_key, &photo_key)?;

		// Create photo data/properties and encrypt it
		let data = PhotoData {
			hash: photo.image.hash.clone(),
			width: photo.image.width,
			height: photo.image.height,
			content_type: "image/jpeg".to_string(), // TODO
			exif: photo.exif.clone(),
		};
		let data_json = serde_json::to_string(&data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&photo_key, data_bytes)?;

		Ok(request::UploadPhoto {
			hash: photo.image.hash.clone(),
			width: photo.image.width,
			height: photo.image.height,
			data: data_encrypt_result.into(),
			key: photo_key_encrypt_result.into(),
			key_hash: photo_key_hash,
			thumbnail_nonce: String::new(),
			preview_nonce: String::new(),
			original_nonce: String::new(),
		})
	}

	async fn get_album(&self, private_key: &[u8], id: &str) -> Result<album::Album> {
		let albums = self.get_albums(private_key).await?;
		let album = albums.into_iter().find(|album| album.get_id() == id).ok_or("Album not found")?;

		Ok(album)
	}

	async fn get_album_using_key_access_proof(&self, id: &str, album_key: &[u8]) -> Result<album::Album> {
		let album_encrypted = self.http_client.get_album_using_key_access_proof(id, album_key).await?;
		let album = Album::from_encrypted(album_encrypted, album_key)?;

		Ok(album)
	}

	pub async fn get_album_full(&self, private_key: &[u8], id: &str) -> Result<JsAlbumFull> {
		let album = self.get_album(private_key, id).await?;
		let album = album.as_js_value();
		let photos = self.http_client.get_photos().await?;

		let mut photos_in_album: Vec<JsAlbumPhoto> = vec![];
		for photo in &photos {
			if album.photos.contains(&photo.id) {
				photos_in_album.push(JsAlbumPhoto {
					id: photo.id.clone(),
					width: photo.width,
					height: photo.height,
					key: None,
				});
			}
		}

		let album = JsAlbumFull {
			id: album.id.clone(),
			title: album.title.clone(),
			tags: album.tags.clone(),
			photos: photos_in_album,
			thumbnail_photo: match album.thumbnail_photo_id.clone() {
				Some(thumbnail_photo_id) => {
					let photo = photos
						.into_iter()
						.find(|photo| photo.id == thumbnail_photo_id)
						.ok_or(format!("Photo not found for thumbnail of album {}", &album.id))?;
					Some(JsAlbumPhoto {
						id: photo.id,
						width: photo.width,
						height: photo.height,
						key: None,
					})
				}
				None => None,
			},
		};

		Ok(album)
	}

	pub async fn get_albums(&self, private_key: &[u8]) -> Result<Vec<album::Album>> {
		let encrypted_albums = self.http_client.get_albums().await?;
		let mut albums: Vec<album::Album> = vec![];

		for album in encrypted_albums {
			let album = album::Album::from_encrypted_with_owner_key(album, private_key)?;
			albums.push(album);
		}

		Ok(albums)
	}

	pub async fn create_album(&self, private_key: &[u8], title: &str, initial_photo_ids: Vec<String>) -> Result<String> {
		let album_key = crate::encryption::symmetric::generate_key();
		let album_key_encrypt_result = crate::encryption::symmetric::encrypt_slice(private_key, &album_key)?;
		let album_key_hash = compute_sha256_hash(&album_key)?;

		let data = album::AlbumData {
			title: title.into(),
			tags: vec![],
			photos: initial_photo_ids,
			thumbnail_photo_id: None,
		};
		let data_json = serde_json::to_string(&data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&album_key, data_bytes)?;

		let body = request::CreateAlbum {
			data: data_encrypt_result.into(),
			key: album_key_encrypt_result.into(),
			key_hash: album_key_hash,
		};

		let result = self.http_client.create_album(&body).await?;
		Ok(result.id)
	}

	pub async fn delete_album(&self, id: &str) -> Result<()> {
		// Delete share for this album (if exists)
		let identifier_hash = Share::get_identifier_hash(&ShareType::Album, id)?;
		let shares = self
			.http_client
			.get_shares(Some(FindSharesFilter {
				identifier_hash: Some(identifier_hash),
			}))
			.await?;

		for share in shares {
			self.http_client.delete_share(&share.id).await?;
		}

		// Delete album itself
		self.http_client.delete_album(id).await
	}

	pub async fn update_album_title_tags(&self, private_key: &[u8], id: &str, title: &str, tags: Vec<String>) -> Result<()> {
		let mut album = self.get_album(private_key, id).await?;

		let mut album_data = album.get_data_mut();
		album_data.title = title.into();
		album_data.tags = tags;

		self.update_album(private_key, id, &album).await
	}

	pub async fn update_album_cover(&self, private_key: &[u8], id: &str, thumbnail_photo_id: &str) -> Result<()> {
		let mut album = self.get_album(private_key, id).await?;

		let mut album_data = album.get_data_mut();
		album_data.thumbnail_photo_id = Some(thumbnail_photo_id.into());

		self.update_album(private_key, id, &album).await
	}

	pub async fn add_photos_to_album(&self, private_key: &[u8], id: &str, photos: &[String]) -> Result<()> {
		let mut album = self.get_album(private_key, id).await?;

		let album_data = album.get_data_mut();
		for id in photos {
			if !album_data.photos.contains(id) {
				album_data.photos.push(id.to_owned());
			}
		}

		self.update_album(private_key, id, &album).await
	}

	/// Remove given photo IDs from album.
	/// Unsets the album's thumbnail if the current thumbnail is one of the photos to remove from album.
	pub async fn remove_photos_from_album(&self, private_key: &[u8], id: &str, photos: &[String]) -> Result<()> {
		let mut album = self.get_album(private_key, id).await?;

		let mut album_data = album.get_data_mut();
		album_data.photos.retain(|id| !photos.contains(id));

		if let Some(thumb_photo_id) = &album_data.thumbnail_photo_id {
			if photos.contains(thumb_photo_id) {
				album_data.thumbnail_photo_id = None;
			}
		}

		self.update_album(private_key, id, &album).await
	}

	/// Creates or updates a share.
	///
	/// * `item_id` - ID of the item (e.g. an album) to create a share for.
	pub async fn upsert_share(&self, private_key: &[u8], share_type: ShareType, item_id: &str, password: &str) -> Result<String> {
		let existing_share_for_album = self.find_share(private_key, &share_type, item_id).await?;

		let data: ShareData = match share_type {
			ShareType::Album => {
				let album = self.get_album(private_key, item_id).await?;
				let album_data = album.get_data();

				let mut find_request: Vec<FindEntity> = vec![];
				for photo in &album_data.photos {
					find_request.push(FindEntity {
						id: photo.to_string(),
						key_hash: None,
					});
				}

				let album_photos_encrypted = self.http_client.find_photos_full(&find_request).await?;
				let mut album_photos: Vec<Photo> = vec![];
				for photo in album_photos_encrypted {
					let photo = Photo::from_encrypted_with_owner_key(photo, private_key)?;
					album_photos.push(photo);
				}

				album.create_share_data(private_key, &album_photos)?
			}
		};

		let share = Self::create_share(private_key, share_type, item_id, password, &data)?;

		if let Some(existing_share) = existing_share_for_album {
			let id = existing_share.get_id();
			self.http_client.update_share(id, &share).await?;
			Ok(id.into())
		} else {
			self.http_client.create_share(&share).await
		}
	}

	/// Create a share for an item
	///
	/// * `item_id` - ID of the item (e.g. an album) to create a share for.
	fn create_share(
		private_key: &[u8],
		share_type: ShareType,
		item_id: &str,
		password: &str,
		share_data: &ShareData,
	) -> Result<request::UpsertShare> {
		let identifier_hash = Share::get_identifier_hash(&share_type, item_id)?;
		let salt = &identifier_hash;
		let share_key = crate::encryption::symmetric::derive_key_from_string(password, salt)?;
		let share_key_encrypt_result = crate::encryption::symmetric::encrypt_slice(private_key, &share_key)?;

		let data_json = serde_json::to_string(&share_data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&share_key, data_bytes)?;

		let password_encrypt_result = crate::encryption::symmetric::encrypt_slice(&share_key, password.as_bytes())?;

		Ok(request::UpsertShare {
			identifier_hash: Share::get_identifier_hash(&share_type, item_id)?,
			type_: share_type,
			password: password_encrypt_result.into(),
			data: data_encrypt_result.into(),
			key: share_key_encrypt_result.into(),
		})
	}

	/// Get shares by decrypting them using owner's key.
	pub async fn get_shares(&self, private_key: &[u8], filters: Option<FindSharesFilter>) -> Result<Vec<Share>> {
		let encrypted_shares = self.http_client.get_shares(filters).await?;
		let mut shares = Vec::new();

		for share in encrypted_shares {
			let share = Share::from_encrypted_with_owner_key(share, private_key)?;
			shares.push(share);
		}

		Ok(shares)
	}

	/// Get a share by decrypting it using owner's key.
	pub async fn get_share(&self, id: &str, private_key: &[u8]) -> Result<Share> {
		let share = self.http_client.get_share(id).await?;
		let share = Share::from_encrypted_with_owner_key(share, private_key)?;

		Ok(share)
	}

	/// Get a share by decrypting it with key derived from given password.
	pub async fn get_share_using_password(&self, id: &str, password: &str) -> Result<Share> {
		let share = self.http_client.get_share(id).await?;
		let share = Self::decrypt_share_using_password(share, password)?;

		Ok(share)
	}

	fn decrypt_share_using_password(share: response::Share, password: &str) -> Result<Share> {
		let salt = &share.identifier_hash;
		let key = encryption::symmetric::derive_key_from_string(password, salt)?;
		let share = Share::from_encrypted(share, &key)?;

		Ok(share)
	}

	/// Get a share by decrypting it using owner's key.
	pub async fn get_album_from_share(&self, share_id: &str, password: &str) -> Result<JsAlbumFull> {
		let share = self.get_share_using_password(share_id, password).await?;
		let data = share.get_data();

		match data {
			ShareData::Album(share_data) => {
				let album = self
					.get_album_using_key_access_proof(&share_data.album_id, &share_data.album_key)
					.await?;
				let album_data = album.get_data();

				let mut photos_proof = vec![];
				let mut photo_keys = HashMap::new();

				for photo in &share_data.photos {
					let photo_key = base64::decode_config(&photo.key, base64::STANDARD)?;
					photos_proof.push(FindEntity {
						id: photo.id.clone(),
						key_hash: Some(compute_sha256_hash(&photo_key)?),
					});
					photo_keys.insert(&photo.id, photo_key);
				}

				let photos = self.http_client.find_photos(&photos_proof).await?;
				let mut js_photos: Vec<JsAlbumPhoto> = Vec::new();
				for photo in &photos {
					js_photos.push(JsAlbumPhoto {
						id: photo.id.clone(),
						width: photo.width,
						height: photo.height,
						key: photo_keys
							.get(&photo.id)
							.map(|bytes| base64::encode_config(bytes, base64::STANDARD)),
					});
				}

				let thumbnail_photo = match album_data.thumbnail_photo_id.clone() {
					Some(thumbnail_photo_id) => {
						let photo = photos
							.iter()
							.cloned()
							.find(|photo| photo.id == thumbnail_photo_id)
							.ok_or(format!("Photo not found for thumbnail of album {}", album.get_id()))?;
						Some(JsAlbumPhoto {
							id: photo.id.clone(),
							width: photo.width,
							height: photo.height,
							key: photo_keys
								.get(&photo.id)
								.map(|bytes| base64::encode_config(bytes, base64::STANDARD)),
						})
					}
					None => None,
				};

				let album = JsAlbumFull {
					id: album.get_id().to_string(),
					title: album_data.title.clone(),
					tags: album_data.tags.clone(),
					photos: js_photos,
					thumbnail_photo,
				};

				Ok(album)
			}
		}
	}

	/// Find a share based on its identifier string
	pub async fn find_share(&self, private_key: &[u8], share_type: &ShareType, id: &str) -> Result<Option<Share>> {
		let identifier_hash = Share::get_identifier_hash(share_type, id)?;
		let shares = self
			.get_shares(
				private_key,
				Some(FindSharesFilter {
					identifier_hash: Some(identifier_hash),
				}),
			)
			.await?;
		Ok(shares.into_iter().next())
	}

	pub async fn delete_share(&self, id: &str) -> Result<()> {
		self.http_client.delete_share(id).await
	}

	async fn update_album(&self, private_key: &[u8], id: &str, album: &Album) -> Result<()> {
		let request_body = album.create_update_request_struct()?;
		self.http_client.update_album(id, &request_body).await?;

		// Refresh album share if there is one
		let album_share = self.find_share(private_key, &ShareType::Album, id).await?;
		if let Some(album_share) = album_share {
			self.upsert_share(private_key, ShareType::Album, id, &album_share.as_js_value().password)
				.await?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::entities::EntityKey;

	#[test]
	fn get_key_from_user_credentials_consistency() {
		let username = "username";
		let password = "password";

		let key_base = UpholiClientHelper::get_key_from_user_credentials(username, password).unwrap();

		// Identical credentials should give same key
		let key = UpholiClientHelper::get_key_from_user_credentials(username, password).unwrap();
		assert_eq!(key_base, key);

		// Any change in credentials should give a different key
		let key = UpholiClientHelper::get_key_from_user_credentials(username, "other_password").unwrap();
		assert_ne!(key_base, key);
		let key = UpholiClientHelper::get_key_from_user_credentials("other_username", password).unwrap();
		assert_ne!(key_base, key);
	}

	#[test]
	fn get_key_from_user_credentials_bad_input() {
		assert!(UpholiClientHelper::get_key_from_user_credentials("username", "").is_err());
		assert!(UpholiClientHelper::get_key_from_user_credentials("", "password").is_err());
		assert!(UpholiClientHelper::get_key_from_user_credentials("", "").is_err());
	}

	#[test]
	fn create_and_decrypt_album_share() {
		let user_private_key = UpholiClientHelper::get_key_from_user_credentials("username", "password").unwrap();
		let album_key = UpholiClientHelper::get_key_from_user_credentials("album", "key").unwrap();
		let share_password = "password";
		let album_id = "abc123";
		let album_photos: Vec<EntityKey> = vec![EntityKey {
			id: String::from("_id"),
			key: String::from("_key"),
		}];

		let share_data: ShareData = ShareData::Album(crate::entities::share::AlbumShareData {
			album_id: album_id.to_string(),
			album_key: album_key.clone(),
			photos: album_photos.clone(),
		});
		let encrypted_share =
			UpholiClientHelper::create_share(&user_private_key, ShareType::Album, album_id, share_password, &share_data).unwrap();

		// Convert the encryped share to the type the server will send in response.
		let http_response_share = response::Share {
			id: String::from("id"),
			user_id: String::from("user_id"),
			identifier_hash: encrypted_share.identifier_hash,
			data: encrypted_share.data,
			type_: ShareType::Album,
			key: encrypted_share.key,
			password: encrypted_share.password,
		};

		let decrypted_share = UpholiClientHelper::decrypt_share_using_password(http_response_share, share_password).unwrap();

		// Share data type should still be an album
		match decrypted_share.get_data() {
			ShareData::Album(album) => {
				assert_eq!(album.album_id, album_id);
				assert_eq!(album.album_key, album_key);
				assert_eq!(album.photos.len(), album_photos.len());

				let matching_elements_count = album
					.photos
					.iter()
					.zip(album_photos.iter())
					.filter(|&(pre, post)| pre.id == post.id && pre.key == post.key)
					.count();
				assert_eq!(album.photos.len(), matching_elements_count);
			}
		}
	}
}
