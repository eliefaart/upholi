use reqwest::StatusCode;
use upholi_lib::http::request::{Login, Register};
use upholi_lib::http::response::{CreateAlbum, PhotoMinimal, UploadPhoto, UserInfo};
use upholi_lib::{EncryptedData, EncryptedKeyInfo, PhotoVariant, ShareType, http::*};
use upholi_lib::result::Result;
use crate::encryption;
use crate::entities::{Entity, Shareable};
use crate::entities::album::{self, Album, AlbumDetailed};
use crate::entities::photo::{Photo, PhotoData};
use crate::entities::share::{AlbumShareData, ShareData};
use crate::images::Image;
use crate::exif::Exif;

/// Wrapper struct containing info about bytes to upload.
pub struct PhotoUploadInfo {
	image: Image,
	exif: Exif
}

impl PhotoUploadInfo {
	/// Try to construct an object from image file bytes
	pub fn try_from_slice(bytes: &[u8]) -> Result<Self> {
		let exif = Exif::parse_from_photo_bytes(&bytes)?;
		let exif_orientation = exif.orientation.unwrap_or(1);

		let image = Image::from_buffer(&bytes, exif_orientation as u8)?;
		Ok(Self {
			image,
			exif
		})
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
pub struct UpholiClientHelper { }

impl UpholiClientHelper {

	pub async fn register(base_url: &str, username: &str, password: &str) -> Result<()> {

		// This will be the master encryption key of the user.
		// We encrypt it using a key derived from the user's password,
		// and the encrypted master key is stored server-side.
		let master_key = encryption::symmetric::generate_key();
		let password_derived_key = encryption::symmetric::derive_key_from_string(password, username)?;
		let key_encrypted = encryption::symmetric::encrypt_slice(&password_derived_key, &master_key)?;

		let body = Register {
			username: username.into(),
			password: password.into(),
			key: key_encrypted.into()
		};

		let url = format!("{}/api/user/register", &base_url).to_owned();
		let client = reqwest::Client::new();
		client.post(&url)
			.json(&body)
			.send().await?;

		Ok(())
	}

	/// Returns the user's master encryption key when login was succesful
	pub async fn login(base_url: &str, username: &str, password: &str) -> Result<Vec<u8>> {
		// derive public/private key pair from password
		// encrypt username with private key
		// send encrypted username to server
		// server will verify by decrypting it using public key

		let body = Login {
			username: username.into(),
			password: password.into()
		};

		let url = format!("{}/api/user/login", &base_url).to_owned();
		let client = reqwest::Client::new();
		let response = client.post(&url)
			.json(&body)
			.send().await?;
		let user: UserInfo = response.json().await?;

		let password_derived_key = encryption::symmetric::derive_key_from_string(&password, &username)?;
		let key = encryption::symmetric::decrypt_data_base64(&password_derived_key, &user.key)?;

		Ok(key)
	}

	pub async fn get_user_info(base_url: &str) -> Result<UserInfo> {
		let url = format!("{}/api/user/info", &base_url).to_owned();
		let client = reqwest::Client::new();
		let response = client
			.get(&url)
			.send().await?;
		let user_info: UserInfo = response
			.json().await?;

		Ok(user_info)
	}

	pub async fn upload_photo(base_url: &str, private_key: &[u8], upload_info: &PhotoUploadInfo) -> Result<String> {
		let mut request_data = Self::get_upload_photo_request_data(&upload_info, &private_key)?;

		let exists = Self::photo_exists(base_url, &request_data.hash).await?;
		if exists {
			// No error, just skipping upload.
			Ok(String::new())
		}
		else {
			// Decrypt photo key
			let owner_key = request_data.keys.iter().find(|key| key.name == crate::OWNER_KEY_NAME).ok_or("Owner key not found")?;
			let photo_key = crate::encryption::symmetric::decrypt_data_base64(private_key, &owner_key.encrypted_key)?;

			// Encrypt photo bytes
			let thumbnail_encrypted = crate::encryption::symmetric::encrypt_slice(&photo_key, &upload_info.bytes_thumbnail())?;
			let preview_encrypted = crate::encryption::symmetric::encrypt_slice(&photo_key, &upload_info.bytes_preview())?;
			let original_encrypted = crate::encryption::symmetric::encrypt_slice(&photo_key, &upload_info.bytes_original())?;

			// Store nonces in request data
			request_data.thumbnail_nonce = thumbnail_encrypted.nonce;
			request_data.preview_nonce = preview_encrypted.nonce;
			request_data.original_nonce = original_encrypted.nonce;

			// Prepare request body
			let multipart = crate::multipart::MultipartBuilder::new()
				.add_bytes("data", &serde_json::to_vec(&request_data)?)
				.add_bytes("thumbnail", &thumbnail_encrypted.bytes)
				.add_bytes("preview", &preview_encrypted.bytes)
				.add_bytes("original", &original_encrypted.bytes)
				.build();

			// Send request
			let url = format!("{}/api/photo", &base_url).to_owned();
			let client = reqwest::Client::new();
			let response = client.post(&url).body(multipart.body)
				.header("Content-Type", multipart.content_type)
				.header("Content-Length", multipart.content_length)
				.send().await?;
			let respone: UploadPhoto = response.json().await?;

			Ok(respone.id)
		}
	}

	/// Check if photo with hash already exists for current user
	pub async fn photo_exists(base_url: &str, hash: &str) -> Result<bool> {
		let url = format!("{}/api/photo?hash={}", &base_url, hash).to_owned();

		let client = reqwest::Client::new();
		let response = client.head(&url).send().await?;

		match response.status() {
			StatusCode::NO_CONTENT => Ok(true),
			StatusCode::NOT_FOUND => Ok(false),
			status_code => Err(Box::from(format!("Unexpected response code: {}", status_code)))
		}
	}

	pub async fn get_photos(base_url: &str) -> Result<Vec<response::PhotoMinimal>> {
		let url = format!("{}/api/photos", &base_url);
		let response = reqwest::get(url).await?;
		let photos = response.json::<Vec<response::PhotoMinimal>>().await?;

		Ok(photos)
	}

	pub async fn get_photo(base_url: &str, private_key: &[u8], id: &str) -> Result<Photo> {
		let photo = UpholiClientHelper::get_photo_encrypted(base_url, id).await?;
		let photo = Photo::from_encrypted(photo, crate::OWNER_KEY_NAME, private_key)?;
		Ok(photo)
	}

	/// Get photo as returned by server.
	pub async fn get_photo_encrypted(base_url: &str, id: &str) -> Result<response::Photo> {
		let url = format!("{}/api/photo/{}", base_url, id);
		let response = reqwest::get(url).await?;
		let encrypted_photo = response.json::<response::Photo>().await?;

		Ok(encrypted_photo)
	}

	pub async fn delete_photos(base_url: &str, private_key: &[u8], ids: &Vec<String>) -> Result<()> {
		// Remove photos from all albums they are part of
		let albums = Self::get_albums(base_url, private_key).await?;
		for album in albums {
			let album_data = album.get_data();
			if album_data.photos.iter().any(|photo| ids.contains(photo)) {
				Self::remove_photos_from_album(base_url, private_key, album.get_id(), ids).await?;
			}
		}

		// Delete photos
		for id in ids {
			let url = format!("{}/api/photo/{}", base_url, id);
			let client = reqwest::Client::new();
			client.delete(url).send().await?;
		}

		Ok(())
	}

	pub async fn get_photo_base64(base_url: &str, private_key: &[u8], id: &str, photo_variant: PhotoVariant) -> Result<String> {
		let url = format!("{}/api/photo/{}/{}", base_url, id, photo_variant.to_string());

		// Get photo bytes
		let response = reqwest::get(url).await?;
		let encrypted_bytes = response.bytes().await?;

		// Decrypt photo bytes
		let photo = Self::get_photo_encrypted(base_url, id).await?;
		let owner_key = photo.keys.iter().find(|key| key.name == crate::OWNER_KEY_NAME).ok_or("Owner key not found")?;
		let photo_key = crate::encryption::symmetric::decrypt_data_base64(private_key, &owner_key.encrypted_key)?;
		let nonce = match photo_variant {
			PhotoVariant::Thumbnail => photo.thumbnail_nonce.as_bytes(),
			PhotoVariant::Preview => photo.preview_nonce.as_bytes(),
			PhotoVariant::Original => photo.original_nonce.as_bytes()
		};
		let bytes = crate::encryption::symmetric::decrypt_slice(&photo_key, nonce, &encrypted_bytes)?;

		Ok(base64::encode_config(&bytes, base64::STANDARD))
	}

	pub async fn get_photo_image_src(base_url: &str, private_key: &[u8], id: &str, photo_variant: PhotoVariant) -> Result<String> {
		let photo = Self::get_photo(base_url, private_key, id).await?;
		let photo_data = photo.get_data();
		let base64 = Self::get_photo_base64(base_url, private_key, id, photo_variant).await?;

		let src = format!("data:{};base64,{}", photo_data.content_type, base64);
		Ok(src)
	}

	/// Get data about photo to send as part of the HTTP request's body
	pub fn get_upload_photo_request_data(photo: &PhotoUploadInfo, private_key: &[u8]) -> Result<request::UploadPhoto> {
		// Generate a key and encrypt it
		let photo_key = crate::encryption::symmetric::generate_key();
		let photo_key_encrypt_result = crate::encryption::symmetric::encrypt_slice(private_key, &photo_key)?;

		// Create photo data/properties and encrypt it
		let data = PhotoData {
			hash: photo.image.hash.clone(),
			width: photo.image.width,
			height: photo.image.height,
			content_type: "image/jpeg".to_string(), // TODO
			exif: crate::exif::Exif {
				manufactorer: photo.exif.manufactorer.to_owned(),
				model: photo.exif.model.to_owned(),
				aperture: photo.exif.aperture.to_owned(),
				exposure_time: photo.exif.exposure_time.to_owned(),
				iso: photo.exif.iso,
				focal_length: photo.exif.focal_length,
				focal_length_35mm_equiv: photo.exif.focal_length_35mm_equiv,
				orientation: photo.exif.orientation,
				date_taken: photo.exif.date_taken,
				gps_latitude: photo.exif.gps_latitude,
				gps_longitude: photo.exif.gps_longitude,
			}
		};
		let data_json = serde_json::to_string(&data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&photo_key, data_bytes)?;

		Ok(request::UploadPhoto {
			hash: photo.image.hash.clone(),
			width: photo.image.width,
			height: photo.image.height,
			data: data_encrypt_result.into(),
			keys: vec!{
				EncryptedKeyInfo {
					name: crate::OWNER_KEY_NAME.into(),
					encrypted_key: EncryptedData {
						base64: base64::encode_config(photo_key_encrypt_result.bytes, base64::STANDARD),
						nonce: photo_key_encrypt_result.nonce,
						format_version: 1
					}
				}
			},
			thumbnail_nonce: String::new(),
			preview_nonce: String::new(),
			original_nonce: String::new()
		})
	}

	async fn get_album(base_url: &str, private_key: &[u8], id: &str) -> Result<album::Album> {
		let albums = Self::get_albums(base_url, private_key).await?;
		let album = albums.into_iter()
			.find(|album| album.get_id() == id)
			.ok_or("Album not found")?;

		Ok(album)
	}

	pub async fn get_album_full(base_url: &str, private_key: &[u8], id: &str) -> Result<AlbumDetailed> {
		let album = Self::get_album(base_url, private_key, id).await?;
		let album = album.as_js_value();
		let photos = Self::get_photos(base_url).await?;

		let mut photos_in_album: Vec<PhotoMinimal> = vec!{};
		for photo in &photos {
			if album.photos.contains(&photo.id) {
				photos_in_album.push(photo.clone());
			}
		}

		let album = AlbumDetailed {
			id: album.id.clone(),
			title: album.title.clone(),
			tags: album.tags.clone(),
			photos: photos_in_album,
			thumbnail_photo: match album.thumbnail_photo_id.clone() {
				Some(thumbnail_photo_id) => photos.into_iter().find(|photo| photo.id == thumbnail_photo_id),
				None => None
			}
		};

		Ok(album)
	}

	pub async fn get_album_from_token(base_url: &str, token: &str, password: &str) -> Result<AlbumDetailed> {
		let (_share_type, id) = Self::parse_share_token(token)?;

		// Share type can only be album for now
		let url = format!("{}/api/album/{}", &base_url, &id);
		let response = reqwest::get(url).await?;
		let album = response.json::<response::Album>().await?;

		let key = encryption::symmetric::derive_key_from_string(password, &id)?;
		let album = album::Album::from_encrypted(album, &token, &key)?;
		let album = album.as_js_value();

		let album = AlbumDetailed {
			id: album.id.clone(),
			title: album.title.clone(),
			tags: album.tags.clone(),
			photos: vec!{},
			thumbnail_photo: None
		};

		Ok(album)
	}

	/// Parse a share token, returns the type of share, and its ID
	fn parse_share_token(token: &str) -> Result<(String, String)> {
		let token = base64::decode_config(token, base64::STANDARD)?;
		let token = String::from_utf8(token)?;
		let parts: Vec<&str> = token.split(":").collect();
		let token_type = parts[0].to_owned();
		let id = parts[1].to_owned();

		Ok((token_type, id))
	}

	pub async fn get_albums(base_url: &str, private_key: &[u8]) -> Result<Vec<album::Album>> {
		let encrypted_albums = Self::get_encrypted_albums(base_url).await?;
		let mut albums: Vec<album::Album> = vec!{};

		for album in encrypted_albums {
			let album = album::Album::from_encrypted(album, crate::OWNER_KEY_NAME, private_key)?;
			albums.push(album);
		}

		Ok(albums)
	}

	pub async fn create_album(base_url: &str, private_key: &[u8], title: &str, initial_photo_ids: Vec<String>) -> Result<String> {
		let url = format!("{}/api/album", &base_url).to_owned();

		let album_key = crate::encryption::symmetric::generate_key();
		let album_key_encrypt_result = crate::encryption::symmetric::encrypt_slice(private_key, &album_key)?;

		let data = album::AlbumData {
			title: title.into(),
			tags: vec!{},
			photos: initial_photo_ids,
			thumbnail_photo_id: None
		};
		let data_json = serde_json::to_string(&data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&album_key, data_bytes)?;

		let body = request::CreateAlbum {
			data: data_encrypt_result.into(),
			keys: vec!{
				EncryptedKeyInfo {
					name: crate::OWNER_KEY_NAME.into(),
					encrypted_key: EncryptedData {
						base64: base64::encode_config(album_key_encrypt_result.bytes, base64::STANDARD),
						nonce: album_key_encrypt_result.nonce,
						format_version: 1
					}
				}
			}
		};

		let client = reqwest::Client::new();
		let response = client.post(&url)
			.json(&body)
			.send().await?;
		let response_body: CreateAlbum = response.json().await?;

		Ok(response_body.id)
	}

	pub async fn delete_album(base_url: &str, id: &str) -> Result<()> {
		let url = format!("{}/api/album/{}", &base_url, &id).to_owned();
		let client = reqwest::Client::new();
		client.delete(&url).send().await?;

		Ok(())
	}

	pub async fn update_album_title_tags(base_url: &str, private_key: &[u8], id: &str, title: &str, tags: Vec<String>) -> Result<()> {
		let mut album = Self::get_album(base_url, private_key, id).await?;

		let mut album_data = album.get_data_mut();
		album_data.title = title.into();
		album_data.tags = tags;

		Self::update_album(base_url, id, &album).await
	}

	pub async fn update_album_cover(base_url: &str, private_key: &[u8], id: &str, thumbnail_photo_id: &str) -> Result<()> {
		let mut album = Self::get_album(base_url, private_key, id).await?;

		let mut album_data = album.get_data_mut();
		album_data.thumbnail_photo_id = Some(thumbnail_photo_id.into());

		Self::update_album(base_url, id, &album).await
	}

	pub async fn add_photos_to_album(base_url: &str, private_key: &[u8], id: &str, photos: &[String]) -> Result<()> {
		let mut album = Self::get_album(base_url, private_key, id).await?;

		let album_data = album.get_data_mut();
		for id in photos {
			if !album_data.photos.contains(&id) {
				album_data.photos.push(id.to_owned());
			}
		}

		Self::update_album(base_url, id, &album).await
	}

	/// Remove given photo IDs from album.
	/// Unsets the album's thumbnail if the current thumbnail is one of the photos to remove from album.
	pub async fn remove_photos_from_album(base_url: &str, private_key: &[u8], id: &str, photos: &[String]) -> Result<()> {
		let mut album = Self::get_album(base_url, private_key, id).await?;

		let mut album_data = album.get_data_mut();
		album_data.photos.retain(|id| !photos.contains(id));

		if let Some(thumb_photo_id) = &album_data.thumbnail_photo_id {
			if photos.contains(&thumb_photo_id) {
				album_data.thumbnail_photo_id = None;
			}
		}

		Self::update_album(base_url, id, &album).await
	}

	/// Creates or updates a share.
	pub async fn upsert_share(base_url: &str, private_key: &[u8], type_: ShareType, id: &str, password: &str) -> Result<String> {
		let url = format!("{}/api/share", &base_url).to_owned();

		let share_key = crate::encryption::symmetric::derive_key_from_string(&password, id)?;
		let share_key_encrypt_result = crate::encryption::symmetric::encrypt_slice(private_key, &share_key)?;

		let data: ShareData = match type_ {
			ShareType::Album => {
				let album = Self::get_album(base_url, private_key, id).await?;
				album.create_share_data(&private_key)?
			}
		};

		let data_json = serde_json::to_string(&data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&share_key, data_bytes)?;

		let body = request::CreateShare {
			data: data_encrypt_result.into(),
			key: EncryptedKeyInfo {
				name: crate::OWNER_KEY_NAME.into(),
				encrypted_key: EncryptedData {
					base64: base64::encode_config(share_key_encrypt_result.bytes, base64::STANDARD),
					nonce: share_key_encrypt_result.nonce,
					format_version: 1
				}
			}
		};

		let client = reqwest::Client::new();
		let response = client.post(&url)
			.json(&body)
			.send().await?;
		let response_body: response::CreateShare = response.json().await?;

		Ok(response_body.id)
	}

	/// Deletes a share.
	pub async fn delete_share(base_url: &str, id: &str) -> Result<()> {
		let url = format!("{}/api/share/{}", &base_url, &id).to_owned();
		let client = reqwest::Client::new();
		client.delete(&url).send().await?;

		Ok(())
	}

	async fn update_album(base_url: &str, id: &str, album: &Album) -> Result<()> {
		let request_body = album.create_update_request_struct()?;

		let url = format!("{}/api/album/{}", base_url, id).to_owned();
		let client = reqwest::Client::new();
		client.put(&url)
			.json(&request_body)
			.send().await?;

		Ok(())
	}

	async fn get_encrypted_albums(base_url: &str) -> Result<Vec<response::Album>> {
		let url = format!("{}/api/albums", &base_url);
		let response = reqwest::get(url).await?;
		let albums = response.json::<Vec<response::Album>>().await?;
		Ok(albums)
	}
}