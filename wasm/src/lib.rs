use std::error::Error;
use exif::Exif;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use uuid::Uuid;
use wasm_bindgen_futures::future_to_promise;
use serde::{Deserialize,Serialize};
use reqwest::multipart;

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
pub fn generate_aes256_key() -> js_sys::Uint8Array {
	let key = Uuid::new_v4().to_simple().to_string();
	js_sys::Uint8Array::from(&key.into_bytes()[..])
}

#[wasm_bindgen]
pub fn aes256_encrypt(key: &[u8], nonce: String, buffer: &[u8]) -> js_sys::Uint8Array {
	let result = aes256::encrypt(key, &nonce.into_bytes(), buffer);
	match result {
		Ok(bytes) => js_sys::Uint8Array::from(&bytes[..]),
		Err(_) => js_sys::Uint8Array::from(&buffer[0..0])
	}
}

#[wasm_bindgen]
pub fn aes256_decrypt(key: &[u8], nonce: String, buffer: &[u8]) -> js_sys::Uint8Array {
	let result = aes256::decrypt(key, &nonce.into_bytes(), buffer);
	match result {
		Ok(bytes) => js_sys::Uint8Array::from(&bytes[..]),
		Err(_) => js_sys::Uint8Array::from(&buffer[0..0])
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
struct UpholiPhotoMinimal {
	id: String,
	width: u32,
	height: u32
}

// #[wasm_bindgen]
// pub struct UpholiPhoto {
// 	id: String,
// 	width: u32,
// 	height: u32,
// 	name: String,
// 	contentType: String,
// 	createdOn: String, // date
// 	hash: String,
// 	exif: UpholiExif,
// }

// #[wasm_bindgen]
// pub struct UpholiExif { }

// #[wasm_bindgen(js_name = getPhotos)]
// pub async fn get_photos() -> Array {
// 	let vec: Vec<u32> = Vec::new();
// 	vec.into()
// }

mod request {
	use serde::{Serialize, Deserialize};
	use crate::aes256;

	#[derive(Deserialize, Serialize)]
	pub struct UploadPhoto {
		/// Encrypted data, contains width, height, exif, etc
		pub data: EncryptedData,
		pub data_version: u8,
		/// Key that all data and file bytes of this photo is encrypted with. This key is encrypted with the owner's private key.
		pub key: EncryptedData,

		pub share_keys: Vec<ShareKey>
	}

	#[derive(Deserialize, Serialize)]
	pub struct ShareKey {
		id: String,
		key: EncryptedData
	}

	#[derive(Deserialize, Serialize)]
	pub struct PhotoData {
		pub width: u32,
		pub height: u32,
		pub content_type: String,
		pub exif: crate::exif::Exif
	}

	#[derive(Deserialize, Serialize)]
	pub struct EncryptedData {
		pub nonce: String,
		pub data: String
	}

	impl UploadPhoto {
		pub fn from_image(image: &crate::PhotoUploadInfo, private_key: &[u8]) -> crate::Result<Self> {

			// Generate a key and encrypt it
			let photo_key = aes256::generate_key();
			let photo_key_nonce = aes256::generate_nonce();
			let photo_key_encrypted = aes256::encrypt(private_key, &photo_key_nonce, &photo_key)?;

			// Create photo data/properties and encrypt it
			let data = PhotoData {
				width: image.image.width,
				height: image.image.height,
				content_type: "".to_string(),
				exif: crate::exif::Exif {
					manufactorer: image.exif.manufactorer.to_owned(),
					model: image.exif.model.to_owned(),
					aperture: image.exif.aperture.to_owned(),
					exposure_time: image.exif.exposure_time.to_owned(),
					iso: image.exif.iso,
					focal_length: image.exif.focal_length,
					focal_length_35mm_equiv: image.exif.focal_length_35mm_equiv,
					orientation: image.exif.orientation,
					date_taken: image.exif.date_taken,
					gps_latitude: image.exif.gps_latitude,
					gps_longitude: image.exif.gps_longitude,
				}
			};
			let data_json = serde_json::to_string(&data).unwrap();
			let data_bytes = data_json.as_bytes();
			let data_nonce = aes256::generate_nonce();
			let data_encrypted = aes256::encrypt(&photo_key, &photo_key_nonce, data_bytes)?;

			Ok(UploadPhoto {
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
}


#[wasm_bindgen]
pub struct UpholiClient {
	base_url: String,
	/// The master private key of current session
	private_key: String
}

/// Client for Upholi server.
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
		let url = format!("{}/api/photo_new", &self.base_url).to_owned();

		future_to_promise(async move {
			match request::UploadPhoto::from_image(&image, &private_key) {
				Ok(request_data) => {
					match serde_json::to_string(&request_data) {
						Ok(request_data) => {
							let client = reqwest::Client::new();
							let form = reqwest::multipart::Form::new()
								.text("data", request_data)
								.part("thumbnail", multipart::Part::bytes(image.bytes_thumbnail()))
								.part("preview", multipart::Part::bytes(image.bytes_preview()))
								.part("original", multipart::Part::bytes(image.bytes())) //.file_name("thumbnail").mime_str("image/jpg"));
								;

							// TODO: Perhaps I should do this in 4 seperate requests:
							//	- POST /photo						to create
							//	- POST /photo/{id}/thumbnail		to upload file bytes
							//	- POST /photo/{id}/preview			to upload file bytes
							//	- POST /photo/{id}/original			to upload file bytes
							match client.post(url)
								//.json(&request_data)
								//.body(request_data)
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
					match response.json::<Vec<UpholiPhotoMinimal>>().await {
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
}