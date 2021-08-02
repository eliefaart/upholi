use std::{error::Error, ops::Deref, sync::Arc};
use exif::Exif;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use serde::{Deserialize,Serialize};
use reqwest::multipart;
use upholi_lib::http::*;
use upholi_lib::result::Result;
use base64::{STANDARD, display::Base64Display, encode};

use crate::aes256::{decrypt, encrypt};

/*
 * Info on async functions within struct implementations:
 * https://github.com/rustwasm/wasm-bindgen/issues/1858
 */

mod error;
mod aes256;
mod images;
mod exif;
mod photos;
mod encryption;

// https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm

// One time needed in ../app/:
// npm install --save ..\wasm\pkg\

#[wasm_bindgen]
extern "C" {
	#[wasm_bindgen(js_namespace = console)]
	fn log(s: &str);

	#[wasm_bindgen(js_namespace = console)]
	fn error(s: &str);
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
    pub fn bytes(&self) -> Vec<u8> {
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





#[derive(Deserialize, Serialize)]
pub struct PhotoData {
	pub width: u32,
	pub height: u32,
	pub content_type: String,
	pub exif: crate::Exif
}

/// Client for Upholi server.
#[wasm_bindgen]
pub struct UpholiClient {
	base_url: String,
	/// The master private key of current session
	private_key: String,
}

struct UpholiClientInternalHelper { }

impl UpholiClientInternalHelper {
	pub async fn get_photo_base64(base_url: &str, private_key: &[u8], id: &str) -> Result<String> {
		let url = format!("{}/api/photo/{}/thumbnail", base_url, id);

		let response = reqwest::get(url).await?;
		let photo_bytes = response.bytes().await?;

		let photo = Self::get_photo_encrypted(base_url, id).await?;
		let photo_key = encryption::decrypt(private_key, &photo.key)?;
		//let photo_data = encryption::decrypt(&photo_key, &photo.data)?;

		// TODO: This fails to decrypt. Bad key_bytes?
		//let photo_bytes = decrypt(&photo_key, photo.data.nonce.as_bytes(), &photo_bytes)?;

		Ok(base64::encode_config(&photo_bytes, base64::STANDARD))
	}

	pub async fn get_photo_encrypted(base_url: &str, id: &str) -> Result<request::UploadPhoto> {
		let url = format!("{}/api/photo/{}", base_url, id);
		let response = reqwest::get(url).await?;
		let encrypted_photo = response.json::<request::UploadPhoto>().await?;
		Ok(encrypted_photo)
	}

	pub async fn get_photo_data(base_url: &str, private_key: &[u8], id: &str) -> Result<PhotoData> {
		let encrypted_photo = Self::get_photo_encrypted(base_url, id).await?;

		let decypted_key = decrypt(private_key, encrypted_photo.key.nonce.as_bytes(), encrypted_photo.key.base64.as_bytes())?;
		let decrypted_data_json = decrypt(&decypted_key, encrypted_photo.data.nonce.as_bytes(), encrypted_photo.data.base64.as_bytes())?;
		let decrypted_data_json = String::from_utf8(decrypted_data_json)?;

		let data = serde_json::from_str::<PhotoData>(&decrypted_data_json)?;

		Ok(data)
	}
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

	#[wasm_bindgen(js_name = uploadPhoto)]
	pub fn upload_photo(&self, image: PhotoUploadInfo) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			let client = reqwest::Client::new();

			match UpholiClient::get_upload_photo_request_data(&image, &private_key) {
				Ok(request_data) => {
					let url = format!("{}/api/photo", &base_url).to_owned();
					match client.post(&url)
						.json(&request_data)
						.send().await {
						Ok(response) => {
							let response: response::UploadPhoto = response.json().await.unwrap();

							//let thumbnail_base64 = base64::encode_config(&image.bytes_thumbnail(), base64::STANDARD);
							let thumbnail_data = encryption::encrypt_slice(&private_key, &image.bytes_thumbnail()).unwrap();
							let form = reqwest::multipart::Form::new()
								.part("thumbnail", multipart::Part::text(thumbnail_data.base64))
								// .part("preview", multipart::Part::bytes(image.bytes_preview()))
								// .part("original", multipart::Part::bytes(image.bytes())) //.file_name("thumbnail").mime_str("image/jpg"));
								;

							let url = format!("{}/api/photo/{}/thumbnail", &base_url, &response.id).to_owned();
							match client.put(&url)
								.multipart(form)
								.send().await {
								Ok(_) => Ok(JsValue::NULL),
								Err(error) => Err(String::from(format!("{}", error)).into())
							}
						},
						Err(error) => Err(String::from(format!("{}", error)).into())
					}
				},
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = getPhotos)]
	pub fn get_photos(&self) -> js_sys::Promise {
		let url = format!("{}/api/photos", &self.base_url).to_owned();

		future_to_promise(async move {
			match reqwest::get(url).await {
				Ok(response) => {
					match response.json::<Vec<response::PhotoMinimal>>().await {
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
				},
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = getPhotoBase64)]
	pub fn get_photo_base64(&self, id: String) -> js_sys::Promise {
		let private_key = self.private_key.as_bytes().to_owned();
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientInternalHelper::get_photo_base64(&base_url, &private_key, &id).await {
				Ok(base64) => Ok(JsValue::from(base64)),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	// async fn get_photo_base64_2(base_url: &str, id: &str) -> Result<String> {
	// 	let url = format!("{}/api/photo/{}/thumbnail", &base_url, id);

	// 	let response = reqwest::get(url).await?;
	// 	let photo_bytes = response.bytes().await?;

	// 	Ok("".into())
	// }

	// async fn get_photo_encrypted(base_url: &str, id: &str) -> Result<request::UploadPhoto> {
	// 	let url = format!("{}/api/photo/{}", base_url, id);
	// 	let response = reqwest::get(url).await?;
	// 	let encrypted_photo = response.json::<request::UploadPhoto>().await?;
	// 	Ok(encrypted_photo)
	// }

	// async fn get_photo_data(base_url: &str, id: &str) -> Result<PhotoData> {
	// 	let encrypted_photo = Self::get_photo_encrypted(base_url, id).await?;

	// 	let decypted_key = decrypt(&self.private_key.as_bytes(), encrypted_photo.key.nonce.as_bytes(), encrypted_photo.key.data.as_bytes())?;
	// 	let decrypted_data_json = decrypt(&decypted_key, encrypted_photo.data.nonce.as_bytes(), encrypted_photo.data.data.as_bytes())?;
	// 	let decrypted_data_json = String::from_utf8(decrypted_data_json)?;

	// 	let data = serde_json::from_str::<PhotoData>(&decrypted_data_json)?;

	// 	Ok(data)
	// }
	// pub fn get_array(&self) -> Vec<JsValue> {
	// 	let photos: Vec<UpholiPhotoMinimal> = vec!{};
	// 	photos.iter().map(JsValue::from).collect()
	// }

	// pub async fn get_photo(&mut self, id: String) {}
	// pub async fn get_photo_bytes_original(&mut self, id: String) {}
	// pub async fn get_photo_bytes_preview(&mut self, id: String) {}
	// pub async fn get_photo_bytes_thumbnail(&mut self, id: String) {}

	// pub async fn get_albums(&mut self) {}
	// pub async fn get_album(&mut self, id: String) {}
	// pub async fn insert_album(&mut self) {}
	// pub async fn update_album(&mut self) {}


	fn get_upload_photo_request_data(photo: &crate::PhotoUploadInfo, private_key: &[u8]) -> Result<request::UploadPhoto> {
		// Generate a key and encrypt it
		let photo_key = aes256::generate_key();
		let photo_key_nonce = aes256::generate_nonce();
		let photo_key_encrypted = aes256::encrypt(private_key, &photo_key_nonce, &photo_key)?;

		// Create photo data/properties and encrypt it
		let data = PhotoData {
			width: photo.image.width,
			height: photo.image.height,
			content_type: "".to_string(),
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
		let data_nonce = aes256::generate_nonce();
		let data_encrypted = aes256::encrypt(&photo_key, &data_nonce, data_bytes)?;

		Ok(request::UploadPhoto {
			width: photo.image.width,
			height: photo.image.height,
			data_version: 1,
			data: EncryptedData {
				nonce: String::from_utf8(data_nonce)?,
				base64: base64::encode_config(&data_encrypted, base64::STANDARD)
			},
			key: EncryptedData {
				nonce: String::from_utf8(photo_key_nonce)?,
				base64: base64::encode_config(&photo_key_encrypted, base64::STANDARD)
			},
			share_keys: vec!{}
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn process_image() {
		let key = b"e0ca4c29d5504e8daa8c52e873e66f71";
		let nonce = b"452b4dd698de";
		let bytes = b"message";
		// let minipng = [137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 1,
		// 	0, 0, 0, 1, 8, 0, 0, 0, 0, 58, 126, 155, 85, 0, 0, 0, 10, 73, 68, 65, 84,
		// 	8, 215, 99, 248, 15, 0, 1, 1, 1, 0, 27, 182, 238, 86, 0, 0, 0, 0, 73, 69,
		// 	78, 68, 174, 66, 96, 130];
		//let photo_bytes = b"GIF89a\x01\x00\x01\x00\x00\xff\x00,\x00\x00\x00\x00\x01\x00\x01\x00\x00\x02\x00;";
	}
}