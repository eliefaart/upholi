use crate::entities::Entity;
use crate::entities::album::{self, AlbumData, AlbumDetailed};
use crate::entities::photo::{Photo, PhotoData};
use crate::images::Image;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use js_sys::{Array, JsString};
use upholi_lib::http::response::{CreateAlbum, PhotoMinimal};
use upholi_lib::{EncryptedData, PhotoVariant, http::*};
use upholi_lib::result::Result;
use crate::exif::Exif;

/*
 * Info on async functions within struct implementations:
 * https://github.com/rustwasm/wasm-bindgen/issues/1858
 *
 * https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
 *
 * One time needed in ../app/:
 * npm install --save ..\wasm\pkg\
 */

/// Wrapper struct containing info about bytes to upload.
struct PhotoUploadInfo {
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

/// Client for Upholi server.
#[wasm_bindgen]
pub struct UpholiClient {
	base_url: String,
	/// The master private key of current session
	private_key: String,
}

#[wasm_bindgen]
impl UpholiClient {
	#[wasm_bindgen(constructor)]
	pub fn new(base_url: String, private_key: String) -> UpholiClient {
		UpholiClient {
			base_url,
			private_key
		}
	}

	/// Get all photos of current user.
	#[wasm_bindgen(js_name = getPhotos)]
	pub fn get_photos(&self) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::get_photos(&base_url).await {
				Ok(photos) => {
					let mut js_array_photos: Vec<JsValue> = Vec::new();

					for photo in photos {
						let photo = JsValue::from_serde(&photo).unwrap_throw();
						js_array_photos.push(photo);
					}

					let js_array_photos = JsValue::from(js_array_photos.iter().collect::<Array>());
					Ok(js_array_photos)
				},
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	/// Get photo data
	#[wasm_bindgen(js_name = getPhoto)]
	pub fn get_photo(&self, id: String) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::get_photo(&base_url, &private_key, &id).await {
				Ok(photo) => {
					match serde_json::to_string(photo.get_data()) {
						Ok(json) => Ok(JsValue::from(json)),
						Err(error) => Err(String::from(format!("{}", error)).into())
					}
				},
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	/// Upload/Create a photo
	#[wasm_bindgen(js_name = uploadPhoto)]
	pub fn upload_photo(&self, bytes: Vec<u8>) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			let upload_info = PhotoUploadInfo::try_from_slice(&bytes).unwrap_throw();
			match UpholiClientHelper::upload_photo(&base_url, &private_key, &upload_info).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	/// Get a base64 string of a photo's thumbnail image
	#[wasm_bindgen(js_name = getPhotoThumbnailBase64)]
	pub fn get_photo_thumbnail_base64(&self, id: String) -> js_sys::Promise {
		Self::get_photo_base64(&self, id, PhotoVariant::Thumbnail)
	}

	/// Get a base64 string of a photo's preview image
	#[wasm_bindgen(js_name = getPhotoPreviewBase64)]
	pub fn get_photo_preview_base64(&self, id: String) -> js_sys::Promise {
		Self::get_photo_base64(&self, id, PhotoVariant::Preview)
	}

	/// Get a base64 string of photo's original file
	#[wasm_bindgen(js_name = getPhotoOriginalBase64)]
	pub fn get_photo_original_base64(&self, id: String) -> js_sys::Promise {
		Self::get_photo_base64(&self, id, PhotoVariant::Original)
	}

	/// Get a base64 string of a photo variant
	fn get_photo_base64(&self, id: String, photo_variant: PhotoVariant) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::get_photo_base64(&base_url, &private_key, &id, photo_variant).await {
				Ok(base64) => Ok(JsValue::from(base64)),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	/// Get a base64 string of a photo's thumbnail image
	#[wasm_bindgen(js_name = getPhotoThumbnailImageSrc)]
	pub fn get_photo_thumbnail_image_src(&self, id: String) -> js_sys::Promise {
		Self::get_photo_image_src(&self, id, PhotoVariant::Thumbnail)
	}

	/// Get a base64 string of a photo's preview image
	#[wasm_bindgen(js_name = getPhotoPreviewImageSrc)]
	pub fn get_photo_preview_image_src(&self, id: String) -> js_sys::Promise {
		Self::get_photo_image_src(&self, id, PhotoVariant::Preview)
	}

	/// Get a base64 string of photo's original file
	#[wasm_bindgen(js_name = getPhotoOriginalImageSrc)]
	pub fn get_photo_original_image_src(&self, id: String) -> js_sys::Promise {
		Self::get_photo_image_src(&self, id, PhotoVariant::Original)
	}

	/// Get a string of a photo variant that can be used within an HTML image element's src attribute
	fn get_photo_image_src(&self, id: String, photo_variant: PhotoVariant) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::get_photo_image_src(&base_url, &private_key, &id, photo_variant).await {
				Ok(base64) => Ok(JsValue::from(base64)),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	/// Permanently delete a photo
	#[wasm_bindgen(js_name = deletePhoto)]
	pub fn delete_photo(&self, id: String) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::delete_photo(&base_url,&id).await {
				Ok(_) => Ok(JsValue::UNDEFINED),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = getAlbums)]
	pub fn get_albums(&mut self) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::get_albums(&base_url, &private_key).await {
				Ok(albums) => {
					let mut js_array: Vec<JsValue> = Vec::new();

					for album in albums {
						let album = JsValue::from_serde(album.into_js_value()).unwrap_throw();
						js_array.push(album);
					}

					let js_array = JsValue::from(js_array.iter().collect::<Array>());
					Ok(js_array)
				},
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = getAlbum)]
	pub fn get_album(&mut self, id: String) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::get_album_full(&base_url, &private_key, &id).await {
				Ok(album) => {
					match serde_json::to_string(&album) {
						Ok(json) => Ok(JsValue::from(json)),
						Err(error) => Err(String::from(format!("{}", error)).into())
					}
				},
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = createAlbum)]
	pub fn create_album(&mut self, title: String, initial_photo_ids: Box<[JsString]>) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let initial_photo_ids = initial_photo_ids.iter().map(|id| id.into()).collect();
			match UpholiClientHelper::create_album(&base_url, &private_key, &title, initial_photo_ids).await {
				Ok(id) => Ok(JsValue::from(id)),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = deleteAlbum)]
	pub fn delete_album(&mut self, id: String) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::delete_album(&base_url, &id).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = updateAlbumTitleTags)]
	pub fn update_album_title_tags(&mut self, id: String, title: String, tags: Box<[JsString]>) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let tags = tags.iter().map(|tag| tag.into()).collect();
			match UpholiClientHelper::update_album_title_tags(&base_url, &private_key, &id, &title, tags).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = updateAlbumCover)]
	pub fn update_album_cover(&mut self, id: String, cover_photo_id: String) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::update_album_cover(&base_url, &private_key, &id, &cover_photo_id).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = addPhotosToAlbum)]
	pub fn add_photos_to_album(&mut self, id: String, photos: Box<[JsString]>) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let photo_ids = photos.iter().map(|photo| photo.into()).collect();
			match UpholiClientHelper::add_photos_to_album(&base_url, &private_key, &id, &photo_ids).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = removePhotosFromAlbum)]
	pub fn remove_photos_from_album(&mut self, id: String, photos: Box<[JsString]>) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			let photo_ids = photos.iter().map(|photo| photo.into()).collect();
			match UpholiClientHelper::remove_photos_from_album(&base_url, &private_key, &id, &photo_ids).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}
}

/// Helper functions for UpholiClient.
/// This object is not exposed outside the wasm.
struct UpholiClientHelper { }

impl UpholiClientHelper {
	pub async fn upload_photo(base_url: &str, private_key: &[u8], upload_info: &PhotoUploadInfo) -> Result<()> {
		let mut request_data = Self::get_upload_photo_request_data(&upload_info, &private_key)?;

		// Decrypt photo key
		let photo_key = crate::encryption::decrypt_data_base64(private_key, &request_data.key)?;

		// Encrypt photo bytes
		let thumbnail_encrypted = crate::encryption::encrypt_slice(&photo_key, &upload_info.bytes_thumbnail())?;
		let preview_encrypted = crate::encryption::encrypt_slice(&photo_key, &upload_info.bytes_preview())?;
		let original_encrypted = crate::encryption::encrypt_slice(&photo_key, &upload_info.bytes_original())?;

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
		client.post(&url).body(multipart.body)
			.header("Content-Type", multipart.content_type)
			.header("Content-Length", multipart.content_length)
			.send().await?;

		Ok(())
	}

	pub async fn get_photos(base_url: &str) -> Result<Vec<response::PhotoMinimal>> {
		let url = format!("{}/api/photos", &base_url).to_owned();
		let response = reqwest::get(url).await?;
		let photos = response.json::<Vec<response::PhotoMinimal>>().await?;

		Ok(photos)
	}

	pub async fn get_photo<'a>(base_url: &str, private_key: &[u8], id: &str) -> Result<Photo> {
		let photo = UpholiClientHelper::get_photo_encrypted(base_url, id).await?;
		let photo = Photo::from_encrypted(photo, private_key)?;
		Ok(photo)
	}

	/// Get photo as returned by server.
	pub async fn get_photo_encrypted(base_url: &str, id: &str) -> Result<response::Photo> {
		let url = format!("{}/api/photo/{}", base_url, id);
		let response = reqwest::get(url).await?;
		let encrypted_photo = response.json::<response::Photo>().await?;

		Ok(encrypted_photo)
	}

	pub async fn delete_photo(base_url: &str, id: &str) -> Result<()> {
		let url = format!("{}/api/photo/{}", base_url, id);
		let client = reqwest::Client::new();
		client.delete(url).send().await?;

		Ok(())
	}

	async fn get_photo_base64(base_url: &str, private_key: &[u8], id: &str, photo_variant: PhotoVariant) -> Result<String> {
		let url = format!("{}/api/photo/{}/{}", base_url, id, photo_variant.to_string());

		// Get photo bytes
		let response = reqwest::get(url).await?;
		let encrypted_bytes = response.bytes().await?;

		// Decrypt photo bytes
		let photo = Self::get_photo_encrypted(base_url, id).await?;
		let photo_key = crate::encryption::decrypt_data_base64(private_key, &photo.key)?;
		let nonce = match photo_variant {
			PhotoVariant::Thumbnail => photo.thumbnail_nonce.as_bytes(),
			PhotoVariant::Preview => photo.preview_nonce.as_bytes(),
			PhotoVariant::Original => photo.original_nonce.as_bytes()
		};
		let bytes = crate::encryption::decrypt_slice(&photo_key, nonce, &encrypted_bytes)?;

		Ok(base64::encode_config(&bytes, base64::STANDARD))
	}

	async fn get_photo_image_src(base_url: &str, private_key: &[u8], id: &str, photo_variant: PhotoVariant) -> Result<String> {
		let photo = Self::get_photo(base_url, private_key, id).await?;
		let photo_data = photo.get_data();
		let base64 = Self::get_photo_base64(base_url, private_key, id, photo_variant).await?;

		let src = format!("data:{};base64,{}", photo_data.content_type, base64);
		Ok(src)
	}

	/// Get data about photo to send as part of the HTTP request's body
	pub fn get_upload_photo_request_data(photo: &PhotoUploadInfo, private_key: &[u8]) -> Result<request::UploadPhoto> {
		// Generate a key and encrypt it
		let photo_key = crate::encryption::generate_key();
		let photo_key_nonce = crate::encryption::generate_nonce();
		let photo_key_encrypted = crate::encryption::encrypt_slice_with_nonce(private_key, &photo_key_nonce, &photo_key)?.bytes;

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
		let data_nonce = crate::encryption::generate_nonce();
		let data_encrypted = crate::encryption::encrypt_slice_with_nonce(&photo_key, &data_nonce, data_bytes)?.bytes;

		Ok(request::UploadPhoto {
			hash: photo.image.hash.clone(),
			width: photo.image.width,
			height: photo.image.height,
			key: EncryptedData {
				nonce: String::from_utf8(photo_key_nonce)?,
				base64: base64::encode_config(&photo_key_encrypted, base64::STANDARD),
				format_version: 1
			},
			data: EncryptedData {
				nonce: String::from_utf8(data_nonce)?,
				base64: base64::encode_config(&data_encrypted, base64::STANDARD),
				format_version: 1
			},
			share_keys: vec!{},
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

	async fn get_album_full(base_url: &str, private_key: &[u8], id: &str) -> Result<AlbumDetailed> {
		let album = Self::get_album(base_url, private_key, id).await?;
		let album = album.into_js_value();
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

	pub async fn get_albums(base_url: &str, private_key: &[u8]) -> Result<Vec<album::Album>> {
		let encrypted_albums = Self::get_encrypted_albums(base_url).await?;
		let mut albums: Vec<album::Album> = vec!{};

		for album in encrypted_albums {
			let album = album::Album::from_encrypted(album, private_key)?;
			albums.push(album);
		}

		Ok(albums)
	}

	pub async fn create_album(base_url: &str, private_key: &[u8], title: &str, initial_photo_ids: Vec<String>) -> Result<String> {
		let url = format!("{}/api/album", &base_url).to_owned();

		let album_key = crate::encryption::generate_key();
		let album_key_nonce = crate::encryption::generate_nonce();
		let album_key_encrypted = crate::encryption::encrypt_slice_with_nonce(private_key, &album_key_nonce, &album_key)?.bytes;

		let data = album::AlbumData {
			title: title.into(),
			tags: vec!{},
			photos: initial_photo_ids,
			thumbnail_photo_id: None
		};
		let data_json = serde_json::to_string(&data)?;
		let data_bytes = data_json.as_bytes();
		let data_nonce = crate::encryption::generate_nonce();
		let data_encrypted = crate::encryption::encrypt_slice_with_nonce(&album_key, &data_nonce, data_bytes)?.bytes;

		let body = request::CreateAlbum {
			key: EncryptedData {
				nonce: String::from_utf8(album_key_nonce)?,
				base64: base64::encode_config(&album_key_encrypted, base64::STANDARD),
				format_version: 1
			},
			data: EncryptedData {
				nonce: String::from_utf8(data_nonce)?,
				base64: base64::encode_config(&data_encrypted, base64::STANDARD),
				format_version: 1
			},
			share_keys: vec!{}
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
		let album = Self::get_album(base_url, private_key, id).await?;
		let mut album_data = album.get_data().clone();
		album_data.title = title.into();
		album_data.tags = tags;

		Self::update_album(base_url, private_key, id, &album_data).await
	}

	pub async fn update_album_cover(base_url: &str, private_key: &[u8], id: &str, thumbnail_photo_id: &str) -> Result<()> {
		let album = Self::get_album(base_url, private_key, id).await?;
		let mut album_data = album.get_data().clone();
		album_data.thumbnail_photo_id = Some(thumbnail_photo_id.into());

		Self::update_album(base_url, private_key, id, &album_data).await
	}

	pub async fn add_photos_to_album(base_url: &str, private_key: &[u8], id: &str, photos: &Vec<String>) -> Result<()> {
		let album = Self::get_album(base_url, private_key, id).await?;
		let mut album_data = album.get_data().clone();
		for id in photos {
			if !album_data.photos.contains(&id) {
				album_data.photos.push(id.to_owned());
			}
		}

		Self::update_album(base_url, private_key, id, &album_data).await
	}

	pub async fn remove_photos_from_album(base_url: &str, private_key: &[u8], id: &str, photos: &Vec<String>) -> Result<()> {
		let album = Self::get_album(base_url, private_key, id).await?;
		let mut album_data = album.get_data().clone();
		album_data.photos = album_data.photos.into_iter().filter(|id| !photos.contains(id)).collect();

		Self::update_album(base_url, private_key, id, &album_data).await
	}

	async fn update_album(base_url: &str, private_key: &[u8], id: &str, album: &AlbumData) -> Result<()> {
		let encrypted_album = Self::get_encrypted_album(base_url, id).await?;
		let album_key = crate::encryption::decrypt_data_base64(private_key, &encrypted_album.key)?;

		let data_json = serde_json::to_string(&album)?;
		let data_bytes = data_json.as_bytes();
		let data_nonce = encrypted_album.data.nonce.into_bytes();
		let data_encrypted = crate::encryption::encrypt_slice_with_nonce(&album_key, &data_nonce, data_bytes)?.bytes;

		let updated_album = request::CreateAlbum {
			key: encrypted_album.key,
			data: EncryptedData {
				nonce: String::from_utf8(data_nonce)?,
				base64: base64::encode_config(&data_encrypted, base64::STANDARD),
				format_version: 1
			},
			share_keys: encrypted_album.share_keys
		};

		let url = format!("{}/api/album/{}", base_url, id).to_owned();
		let client = reqwest::Client::new();
		client.put(&url)
			.json(&updated_album)
			.send().await?;

		Ok(())
	}

	async fn get_encrypted_album(base_url: &str, id: &str) -> Result<response::Album> {
		let albums = Self::get_encrypted_albums(base_url).await?;
		let album = albums.into_iter()
			.find(|album| album.id == id)
			.ok_or("Album not found")?;
		Ok(album)
	}

	async fn get_encrypted_albums(base_url: &str) -> Result<Vec<response::Album>> {
		let url = format!("{}/api/albums", &base_url).to_owned();
		let response = reqwest::get(url).await?;
		let albums = response.json::<Vec<response::Album>>().await?;
		Ok(albums)
	}
}