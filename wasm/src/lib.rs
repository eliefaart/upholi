use std::error::Error;
use exif::Exif;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;
use serde::{Deserialize,Serialize};
use reqwest::multipart;
use upholi_lib::http::*;

/*
 * Info on async functions within struct implementations:
 * https://github.com/rustwasm/wasm-bindgen/issues/1858
 */

mod error;
mod aes256;
mod images;
mod exif;

pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

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
	private_key: String
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
					let url = format!("{}/api/photo_new", &base_url).to_owned();
					match client.post(&url)
						.json(&request_data)
						.send().await {
						Ok(response) => {
							let response: response::UploadPhoto = response.json().await.unwrap();

							let form = reqwest::multipart::Form::new()
								.part("thumbnail", multipart::Part::bytes(image.bytes_thumbnail()))
								// .part("preview", multipart::Part::bytes(image.bytes_preview()))
								// .part("original", multipart::Part::bytes(image.bytes())) //.file_name("thumbnail").mime_str("image/jpg"));
								;

							// let url = format!("{}/api/photo/{}/thumbnail", &base_url, &response.id).to_owned();
							// match client.put(&url)
							// 	.multipart(form)
							// 	.send().await {
							// 	Ok(_) => Ok(JsValue::NULL),
							// 	Err(error) => Err(String::from(format!("{}", error)).into())
							// }

							Ok(JsValue::NULL)
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
		let url = format!("{}/api/photos_new", &self.base_url).to_owned();

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
		let data_encrypted = aes256::encrypt(&photo_key, &photo_key_nonce, data_bytes)?;

		Ok(request::UploadPhoto {
			width: photo.image.width,
			height: photo.image.height,
			data_version: 1,
			data: EncryptedData {
				nonce: String::from_utf8(data_nonce)?,
				data: String::from_utf8_lossy(&data_encrypted).to_string()
			},
			key: EncryptedData {
				nonce: String::from_utf8(photo_key_nonce)?,
				data: String::from_utf8_lossy(&photo_key_encrypted).to_string()
			},
			share_keys: vec!{}
		})
	}
}