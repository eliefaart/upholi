use std::collections::HashMap;

use crate::client::http;
use crate::entities::album::{self, Album, JsAlbumFull, JsAlbumPhoto};
use crate::entities::photo::{Photo, PhotoData};
use crate::entities::share::{Share, ShareData};
use crate::entities::{Entity, EntityWithProof, Shareable};
use crate::exif::Exif;
use crate::hashing::compute_sha256_hash;
use crate::images::Image;
use crate::{encryption, hashing};
use reqwest::StatusCode;
use upholi_lib::http::request::{FindSharesFilter, Login, Register};
use upholi_lib::http::response::{CreateAlbum, ErrorResult, UploadPhoto, UserInfo};
use upholi_lib::result::Result;
use upholi_lib::{http::*, PhotoVariant, ShareType};

/// Wrapper struct containing info about bytes to upload.
pub struct PhotoUploadInfo {
	image: Image,
	exif: Exif,
}

impl PhotoUploadInfo {
	/// Try to construct an object from image file bytes
	pub fn try_from_slice(bytes: &[u8]) -> Result<Self> {
		let exif = Exif::parse_from_photo_bytes(bytes)?;
		let exif_orientation = exif.orientation.unwrap_or(1);

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
pub struct UpholiClientHelper {}

impl UpholiClientHelper {
	pub async fn register(base_url: &str, username: &str, password: &str) -> Result<()> {
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

		let url = format!("{}/api/user/register", &base_url).to_owned();
		let client = reqwest::Client::new();
		let response = client.post(&url).json(&body).send().await?;

		if response.status() == StatusCode::OK {
			Ok(())
		} else {
			let error: ErrorResult = response.json().await?;
			Err(Box::from(error.message))
		}
	}

	/// Returns the user's master encryption key when login was succesful
	pub async fn login(base_url: &str, username: &str, password: &str) -> Result<Vec<u8>> {
		// derive public/private key pair from password
		// encrypt username with private key
		// send encrypted username to server
		// server will verify by decrypting it using public key

		let body = Login {
			username: username.into(),
			password: password.into(),
		};

		let url = format!("{}/api/user/login", &base_url).to_owned();
		let client = reqwest::Client::new();
		let response = client.post(&url).json(&body).send().await?;

		if response.status() == StatusCode::OK {
			let user: UserInfo = response.json().await?;

			let password_derived_key = Self::get_key_from_user_credentials(username, password)?;
			let key = encryption::symmetric::decrypt_data_base64(&password_derived_key, &user.key)?;

			Ok(key)
		} else {
			let error: ErrorResult = response.json().await?;
			Err(Box::from(error.message))
		}
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

	pub async fn get_user_info(base_url: &str) -> Result<UserInfo> {
		let url = format!("{}/api/user/info", &base_url).to_owned();
		let client = reqwest::Client::new();
		let response = client.get(&url).send().await?;
		let user_info: UserInfo = response.json().await?;

		Ok(user_info)
	}

	pub async fn upload_photo(base_url: &str, private_key: &[u8], upload_info: &PhotoUploadInfo) -> Result<String> {
		let mut request_data = Self::get_upload_photo_request_data(upload_info, private_key)?;

		let exists = Self::photo_exists(base_url, &request_data.hash).await?;
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
			let response = client
				.post(&url)
				.body(multipart.body)
				.header("Content-Type", multipart.content_type)
				.header("Content-Length", multipart.content_length)
				.send()
				.await?;
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
			status_code => Err(Box::from(format!("Unexpected response code: {}", status_code))),
		}
	}

	pub async fn get_photos(base_url: &str) -> Result<Vec<response::PhotoMinimal>> {
		let url = format!("{}/api/photos", &base_url);
		let response = reqwest::get(url).await?;
		let photos = response.json::<Vec<response::PhotoMinimal>>().await?;

		Ok(photos)
	}

	pub async fn get_photo(base_url: &str, private_key: &[u8], id: &str, key: &Option<String>) -> Result<Photo> {
		let photo = UpholiClientHelper::get_photo_encrypted(base_url, id, key).await?;
		let photo = match key {
			Some(photo_key) => Photo::from_encrypted(photo, &base64::decode_config(photo_key, base64::STANDARD)?)?,
			None => Photo::from_encrypted_with_owner_key(photo, private_key)?,
		};
		Ok(photo)
	}

	/// Get photo as returned by server.
	pub async fn get_photo_encrypted(base_url: &str, id: &str, key: &Option<String>) -> Result<response::Photo> {
		let mut url = format!("{}/api/photo/{}", base_url, id);

		if let Some(key) = key {
			let key_hash = compute_sha256_hash(&base64::decode_config(key, base64::STANDARD)?)?;
			url = format!("{}?key_hash={}", url, key_hash);
		}

		let response = reqwest::get(url).await?;
		let encrypted_photo = response.json::<response::Photo>().await?;

		Ok(encrypted_photo)
	}

	pub async fn delete_photos(base_url: &str, private_key: &[u8], ids: &[String]) -> Result<()> {
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

	pub async fn get_photo_base64(
		base_url: &str,
		private_key: &[u8],
		id: &str,
		photo_variant: PhotoVariant,
		key: &Option<String>,
	) -> Result<String> {
		let mut url = format!("{}/api/photo/{}/{}", base_url, id, photo_variant.to_string());

		if let Some(key) = key {
			let key_hash = compute_sha256_hash(&base64::decode_config(key, base64::STANDARD)?)?;
			url = format!("{}?key_hash={}", url, key_hash);
		}

		// Get photo bytes
		let response = reqwest::get(url).await?;
		let encrypted_bytes = response.bytes().await?;

		// Decrypt photo bytes
		let photo = Self::get_photo_encrypted(base_url, id, key).await?;
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
		base_url: &str,
		private_key: &[u8],
		id: &str,
		photo_variant: PhotoVariant,
		key: &Option<String>,
	) -> Result<String> {
		if id.is_empty() {
			Ok(String::new())
		} else {
			let photo = Self::get_photo(base_url, private_key, id, key).await?;
			let photo_data = photo.get_data();
			let base64 = Self::get_photo_base64(base_url, private_key, id, photo_variant, key).await?;

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
			},
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

	async fn get_album(base_url: &str, private_key: &[u8], id: &str) -> Result<album::Album> {
		let albums = Self::get_albums(base_url, private_key).await?;
		let album = albums.into_iter().find(|album| album.get_id() == id).ok_or("Album not found")?;

		Ok(album)
	}

	async fn get_album_using_key_access_proof(base_url: &str, id: &str, album_key: &[u8]) -> Result<album::Album> {
		let album_key_hash = compute_sha256_hash(album_key)?;
		let url = format!("{}/api/album/{}?key_hash={}", &base_url, id, &album_key_hash);
		let response = reqwest::get(url).await?;
		let album_encrypted = response.json::<response::Album>().await?;

		let album = Album::from_encrypted(album_encrypted, album_key)?;

		Ok(album)
	}

	pub async fn get_album_full(base_url: &str, private_key: &[u8], id: &str) -> Result<JsAlbumFull> {
		let album = Self::get_album(base_url, private_key, id).await?;
		let album = album.as_js_value();
		let photos = Self::get_photos(base_url).await?;

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

	pub async fn get_albums(base_url: &str, private_key: &[u8]) -> Result<Vec<album::Album>> {
		let encrypted_albums = http::get_albums(base_url).await?;
		let mut albums: Vec<album::Album> = vec![];

		for album in encrypted_albums {
			let album = album::Album::from_encrypted_with_owner_key(album, private_key)?;
			albums.push(album);
		}

		Ok(albums)
	}

	pub async fn create_album(base_url: &str, private_key: &[u8], title: &str, initial_photo_ids: Vec<String>) -> Result<String> {
		let url = format!("{}/api/album", &base_url).to_owned();

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

		let client = reqwest::Client::new();
		let response = client.post(&url).json(&body).send().await?;
		let response_body: CreateAlbum = response.json().await?;

		Ok(response_body.id)
	}

	pub async fn delete_album(base_url: &str, id: &str) -> Result<()> {
		// Delete share for this album (if exists)
		let identifier_hash = Share::get_identifier_hash(&ShareType::Album, id)?;
		let shares = http::get_shares(
			base_url,
			Some(FindSharesFilter {
				identifier_hash: Some(identifier_hash),
			}),
		)
		.await?;

		for share in shares {
			Self::delete_share(base_url, &share.id).await?;
		}

		// Delete album itself
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

		Self::update_album(base_url, private_key, id, &album).await
	}

	pub async fn update_album_cover(base_url: &str, private_key: &[u8], id: &str, thumbnail_photo_id: &str) -> Result<()> {
		let mut album = Self::get_album(base_url, private_key, id).await?;

		let mut album_data = album.get_data_mut();
		album_data.thumbnail_photo_id = Some(thumbnail_photo_id.into());

		Self::update_album(base_url, private_key, id, &album).await
	}

	pub async fn add_photos_to_album(base_url: &str, private_key: &[u8], id: &str, photos: &[String]) -> Result<()> {
		let mut album = Self::get_album(base_url, private_key, id).await?;

		let album_data = album.get_data_mut();
		for id in photos {
			if !album_data.photos.contains(id) {
				album_data.photos.push(id.to_owned());
			}
		}

		Self::update_album(base_url, private_key, id, &album).await
	}

	/// Remove given photo IDs from album.
	/// Unsets the album's thumbnail if the current thumbnail is one of the photos to remove from album.
	pub async fn remove_photos_from_album(base_url: &str, private_key: &[u8], id: &str, photos: &[String]) -> Result<()> {
		let mut album = Self::get_album(base_url, private_key, id).await?;

		let mut album_data = album.get_data_mut();
		album_data.photos.retain(|id| !photos.contains(id));

		if let Some(thumb_photo_id) = &album_data.thumbnail_photo_id {
			if photos.contains(thumb_photo_id) {
				album_data.thumbnail_photo_id = None;
			}
		}

		Self::update_album(base_url, private_key, id, &album).await
	}

	/// Creates or updates a share.
	pub async fn upsert_share(base_url: &str, private_key: &[u8], type_: ShareType, id: &str, password: &str) -> Result<String> {
		let existing_share_for_album = Self::find_share(base_url, private_key, &type_, id).await?;
		let url = match &existing_share_for_album {
			Some(share) => format!("{}/api/share/{}", &base_url, share.get_id()).to_owned(),
			None => format!("{}/api/share", &base_url).to_owned(),
		};

		let salt = "todo";
		let share_key = crate::encryption::symmetric::derive_key_from_string(password, salt)?;
		let share_key_encrypt_result = crate::encryption::symmetric::encrypt_slice(private_key, &share_key)?;

		// TODO: Don't get every single photo, only need the ones included in album
		let photos = Self::get_photos(base_url).await?;
		let mut all_photos: Vec<Photo> = vec![];
		for photo in photos {
			let photo = Self::get_photo(base_url, private_key, &photo.id, &None).await?;
			all_photos.push(photo);
		}

		let data: ShareData = match type_ {
			ShareType::Album => {
				let album = Self::get_album(base_url, private_key, id).await?;
				album.create_share_data(private_key, &all_photos)?
			}
		};
		let data_json = serde_json::to_string(&data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&share_key, data_bytes)?;

		let password_encrypt_result = crate::encryption::symmetric::encrypt_slice(&share_key, password.as_bytes())?;

		let body = request::UpsertShare {
			identifier_hash: Share::get_identifier_hash(&type_, id)?,
			type_,
			password: password_encrypt_result.into(),
			data: data_encrypt_result.into(),
			key: share_key_encrypt_result.into(),
		};

		let client = reqwest::Client::new();
		let request = match existing_share_for_album.is_some() {
			true => client.put(&url),
			false => client.post(&url),
		};
		let response = request.json(&body).send().await?;

		if let Some(existing_share) = existing_share_for_album {
			Ok(existing_share.get_id().into())
		} else {
			let response_body: response::CreateShare = response.json().await?;
			Ok(response_body.id)
		}
	}

	/// Deletes a share.
	pub async fn delete_share(base_url: &str, id: &str) -> Result<()> {
		let url = format!("{}/api/share/{}", &base_url, &id).to_owned();
		let client = reqwest::Client::new();
		client.delete(&url).send().await?;

		Ok(())
	}

	/// Get shares by decrypting them using owner's key.
	pub async fn get_shares(base_url: &str, private_key: &[u8], filters: Option<FindSharesFilter>) -> Result<Vec<Share>> {
		let encrypted_shares = http::get_shares(base_url, filters).await?;
		let mut shares = Vec::new();

		for share in encrypted_shares {
			let share = Share::from_encrypted_with_owner_key(share, private_key)?;
			shares.push(share);
		}

		Ok(shares)
	}

	/// Get a share by decrypting it using owner's key.
	pub async fn get_share(base_url: &str, id: &str, private_key: &[u8]) -> Result<Share> {
		let share = http::get_share(base_url, id).await?;
		let share = Share::from_encrypted_with_owner_key(share, private_key)?;

		Ok(share)
	}

	/// Get a share by decrypting it with key derived from given password.
	pub async fn get_share_using_password(base_url: &str, id: &str, password: &str) -> Result<Share> {
		let share = http::get_share(base_url, id).await?;

		let salt = "todo";
		let key = encryption::symmetric::derive_key_from_string(password, salt)?;
		let share = Share::from_encrypted(share, &key)?;

		Ok(share)
	}

	/// Get a share by decrypting it using owner's key.
	pub async fn get_album_from_share(base_url: &str, share_id: &str, password: &str) -> Result<JsAlbumFull> {
		let share = Self::get_share_using_password(base_url, share_id, password).await?;
		let data = share.get_data();

		match data {
			ShareData::Album(share_data) => {
				let album = Self::get_album_using_key_access_proof(base_url, &share_data.album_id, &share_data.album_key).await?;
				let album_data = album.get_data();

				let mut photos_proof = vec![];
				let mut photo_keys = HashMap::new();

				for photo in &share_data.photos {
					let photo_key = base64::decode_config(&photo.key, base64::STANDARD)?;
					photos_proof.push(EntityWithProof {
						id: photo.id.clone(),
						proof: compute_sha256_hash(&photo_key)?,
					});
					photo_keys.insert(&photo.id, photo_key);
				}

				let photos = http::get_photos_using_key_access_proof(base_url, &photos_proof).await?;
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
	pub async fn find_share(base_url: &str, private_key: &[u8], share_type: &ShareType, id: &str) -> Result<Option<Share>> {
		let identifier_hash = Share::get_identifier_hash(share_type, id)?;
		let shares = Self::get_shares(
			base_url,
			private_key,
			Some(FindSharesFilter {
				identifier_hash: Some(identifier_hash),
			}),
		)
		.await?;
		Ok(shares.into_iter().next())
	}

	async fn update_album(base_url: &str, private_key: &[u8], id: &str, album: &Album) -> Result<()> {
		let request_body = album.create_update_request_struct()?;

		let url = format!("{}/api/album/{}", base_url, id).to_owned();
		let client = reqwest::Client::new();
		client.put(&url).json(&request_body).send().await?;

		// Refresh album share if there is one
		let album_share = Self::find_share(base_url, private_key, &ShareType::Album, id).await?;
		if let Some(album_share) = album_share {
			Self::upsert_share(base_url, private_key, ShareType::Album, id, &album_share.as_js_value().password).await?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

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
}
