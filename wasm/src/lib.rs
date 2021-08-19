use js_sys::Array;
use types::Photo;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use upholi_lib::{PhotoVariant, http::*};
use upholi_lib::result::Result;
use exif::Exif;

use crate::types::Album;

mod types;
mod images;
mod exif;
mod encryption;
mod multipart;
mod hashing;

/*
 * Info on async functions within struct implementations:
 * https://github.com/rustwasm/wasm-bindgen/issues/1858
 *
 * https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
 *
 * One time needed in ../app/:
 * npm install --save ..\wasm\pkg\
 */

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
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
			match UpholiClientHelper::get_photo_data(&base_url, &private_key, &id).await {
				Ok(photo) => {
					match serde_json::to_string(&photo) {
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
	pub fn upload_photo(&self, image: PhotoUploadInfo) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::upload_photo(&base_url, &private_key, &image).await {
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
						let album = JsValue::from_serde(&album).unwrap_throw();
						js_array.push(album);
					}

					let js_array = JsValue::from(js_array.iter().collect::<Array>());
					Ok(js_array)
				},
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = createAlbum)]
	pub fn create_album(&mut self, title: String) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();

		future_to_promise(async move {
			match UpholiClientHelper::create_album(&base_url, &private_key, &title).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	// pub fn update_album(&mut self) {}

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
	pub fn add_photos_to_album(&mut self, id: String) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();
		let photos: Vec<&str> = vec!{};

		future_to_promise(async move {
			match UpholiClientHelper::add_photos_to_album(&base_url, &private_key, &id, &photos).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = removePhotosFromAlbum)]
	pub fn remove_photos_from_album(&mut self, id: String) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();
		let private_key = self.private_key.as_bytes().to_owned();
		let photos: Vec<&str> = vec!{};

		future_to_promise(async move {
			match UpholiClientHelper::remove_photos_from_album(&base_url, &private_key, &id, &photos).await {
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
	pub async fn upload_photo(base_url: &str, private_key: &[u8], image: &PhotoUploadInfo) -> Result<()> {
		let mut request_data = Self::get_upload_photo_request_data(&image, &private_key)?;

		// Decrypt photo key
		let photo_key = encryption::decrypt_data_base64(private_key, &request_data.key)?;

		// Encrypt photo bytes
		let thumbnail_encrypted = encryption::encrypt_slice(&photo_key, &image.bytes_thumbnail())?;
		let preview_encrypted = encryption::encrypt_slice(&photo_key, &image.bytes_preview())?;
		let original_encrypted = encryption::encrypt_slice(&photo_key, &image.bytes_original())?;

		// Store nonces in request data
		request_data.thumbnail_nonce = thumbnail_encrypted.nonce;
		request_data.preview_nonce = preview_encrypted.nonce;
		request_data.original_nonce = original_encrypted.nonce;

		// Prepare request body
		let multipart = multipart::MultipartBuilder::new()
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

	pub async fn get_photo_data(base_url: &str, private_key: &[u8], id: &str) -> Result<Photo> {
		let photo = UpholiClientHelper::get_photo_encrypted(base_url, id).await?;
		let photo_key = encryption::decrypt_data_base64(private_key, &photo.key)?;
		let photo_data = encryption::decrypt_data_base64(&photo_key, &photo.data)?;
		let mut photo_data: Photo = serde_json::from_slice(&photo_data)?;

		photo_data.id = id.into();

		Ok(photo_data)
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
		let photo_key = encryption::decrypt_data_base64(private_key, &photo.key)?;
		let nonce = match photo_variant {
			PhotoVariant::Thumbnail => photo.thumbnail_nonce.as_bytes(),
			PhotoVariant::Preview => photo.preview_nonce.as_bytes(),
			PhotoVariant::Original => photo.original_nonce.as_bytes()
		};
		let bytes = encryption::decrypt_slice(&photo_key, nonce, &encrypted_bytes)?;

		Ok(base64::encode_config(&bytes, base64::STANDARD))
	}

	async fn get_photo_image_src(base_url: &str, private_key: &[u8], id: &str, photo_variant: PhotoVariant) -> Result<String> {
		let photo_data = Self::get_photo_data(base_url, private_key, id).await?;
		let base64 = Self::get_photo_base64(base_url, private_key, id, photo_variant).await?;

		let src = format!("data:{};base64,{}", photo_data.content_type, base64);
		Ok(src)
	}

	/// Get data about photo to send as part of the HTTP request's body
	pub fn get_upload_photo_request_data(photo: &crate::PhotoUploadInfo, private_key: &[u8]) -> Result<request::UploadPhoto> {
		// Generate a key and encrypt it
		let photo_key = encryption::aes256::generate_key();
		let photo_key_nonce = encryption::aes256::generate_nonce();
		let photo_key_encrypted = encryption::aes256::encrypt(private_key, &photo_key_nonce, &photo_key)?;

		// Create photo data/properties and encrypt it
		let data = Photo {
			id: String::new(),
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
		let data_nonce = encryption::aes256::generate_nonce();
		let data_encrypted = encryption::aes256::encrypt(&photo_key, &data_nonce, data_bytes)?;

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

	pub async fn get_album(base_url: &str, private_key: &[u8], id: &str) -> Result<Album> {
		let albums = Self::get_albums(base_url, private_key).await?;
		let album = albums.into_iter()
			.find(|album| album.id == id)
			.ok_or("Album not found")?;

		Ok(album)
	}

	pub async fn get_albums(base_url: &str, private_key: &[u8]) -> Result<Vec<Album>> {
		let url = format!("{}/api/albums", &base_url).to_owned();
		let response = reqwest::get(url).await?;
		let albums = response.json::<Vec<response::Album>>().await?;

		let mut decypted_albums: Vec<Album> = vec!{};

		for album in albums {
			let album_key = encryption::decrypt_data_base64(private_key, &album.key)?;
			let album_data = encryption::decrypt_data_base64(&album_key, &album.data)?;
			let mut album_data: Album = serde_json::from_slice(&album_data)?;

			album_data.id = album.id;

			decypted_albums.push(album_data);
		}

		Ok(decypted_albums)
	}

	pub async fn create_album(base_url: &str, private_key: &[u8], title: &str) -> Result<()> {
		let url = format!("{}/api/album", &base_url).to_owned();

		let album_key = encryption::aes256::generate_key();
		let album_key_nonce = encryption::aes256::generate_nonce();
		let album_key_encrypted = encryption::aes256::encrypt(private_key, &album_key_nonce, &album_key)?;

		let data = Album {
			id: String::new(),
			title: title.into(),
			tags: vec!{},
			photos: vec!{},
			thumbnail_photo_id: None
		};
		let data_json = serde_json::to_string(&data)?;
		let data_bytes = data_json.as_bytes();
		let data_nonce = encryption::aes256::generate_nonce();
		let data_encrypted = encryption::aes256::encrypt(&album_key, &data_nonce, data_bytes)?;

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
		client.post(&url)
			.json(&body)
			.send().await?;

		Ok(())
	}

	pub async fn delete_album(base_url: &str, id: &str) -> Result<()> {
		let url = format!("{}/api/album/{}", &base_url, &id).to_owned();
		let client = reqwest::Client::new();
		client.delete(&url).send().await?;

		Ok(())
	}

	pub async fn update_album_cover(base_url: &str, private_key: &[u8], id: &str, cover_photo_id: &str) -> Result<()> {
		Ok(())
	}

	pub async fn add_photos_to_album(base_url: &str, private_key: &[u8], id: &str, photos: &Vec<&str>) -> Result<()> {
		Ok(())
	}

	pub async fn remove_photos_from_album(base_url: &str, private_key: &[u8], id: &str, photos: &Vec<&str>) -> Result<()> {
		Ok(())
	}

	async fn update_album(base_url: &str, private_key: &[u8], id: &str, modify: fn(album: &mut Album) -> ()) -> Result<()> {
		let mut album = Self::get_album(base_url, private_key, id).await?;
		modify(&mut album);

		// let data_json = serde_json::to_string(&album)?;
		// let data_bytes = data_json.as_bytes();
		// let data_encrypted = encryption::aes256::encrypt(&album_key, &data_nonce, data_bytes)?;

		// let updated_album = request::CreateAlbum {
		// 	key:
		// };

		let url = format!("{}/api/album/{}", base_url, &album.id).to_owned();
		let client = reqwest::Client::new();
		client.post(&url)
			.json(&album)
			.send().await?;

		Ok(())
	}
}

#[wasm_bindgen]
pub struct PhotoUploadInfo {
	image: images::Image,
	exif: Exif
}

#[wasm_bindgen]
impl PhotoUploadInfo {
	#[wasm_bindgen(constructor)]
	pub fn new(bytes: &[u8]) -> PhotoUploadInfo {
		let exif = exif::Exif::parse_from_photo_bytes(bytes);
		match exif {
			Ok(exif) => {
				let exif_orientation = exif.orientation.unwrap_or(1);

				let image = images::Image::from_buffer(bytes, exif_orientation as u8).unwrap();

				PhotoUploadInfo {
					image,
					exif
				}
			},
			Err(error) => panic!("Error parsing exif data: {}", error)
		}
	}

	#[wasm_bindgen(getter)]
    pub fn bytes_original(&self) -> Vec<u8> {
        self.image.bytes_original[..].to_vec()
    }

	#[wasm_bindgen(getter, js_name = bytesPreview)]
    pub fn bytes_preview(&self) -> Vec<u8> {
        self.image.bytes_preview[..].to_vec()
    }

	#[wasm_bindgen(getter, js_name = bytesThumbnail)]
    pub fn bytes_thumbnail(&self) -> Vec<u8> {
        self.image.bytes_thumbnail[..].to_vec()
    }

	#[wasm_bindgen(getter, js_name = exif)]
    pub fn exif(&self) -> JsValue {
        match JsValue::from_serde(&self.exif) {
			Ok(exif) => {
				exif
			},
			Err(error) => JsValue::from(format!("Error serializing: {}", error))
		}
    }
}