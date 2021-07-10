use std::error::Error;
use exif::Exif;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use uuid::Uuid;
use wasm_bindgen_futures::future_to_promise;

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
pub struct ImageUploadInfo {
	image: images::Image,
	exif: Exif
}

#[wasm_bindgen]
impl ImageUploadInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> ImageUploadInfo {
		let exif = exif::Exif::parse_from_photo_bytes(bytes);
		match exif {
			Ok(exif) => {
				let exif_orientation = exif.orientation.unwrap_or(1);

				let image = images::Image::from_buffer(bytes, exif_orientation as u8).unwrap();

				ImageUploadInfo {
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



#[derive(serde::Deserialize, serde::Serialize)]
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

#[wasm_bindgen]
pub struct UpholiClient {
	base_url: String
}

/// Client for Upholi server.
#[wasm_bindgen]
impl UpholiClient {
	#[wasm_bindgen(constructor)]
	pub fn new(base_url: String) -> UpholiClient {
		UpholiClient {
			base_url
		}
	}

	#[wasm_bindgen(js_name = uploadPhoto)]
	pub fn upload_photo(&self, image: ImageUploadInfo) -> js_sys::Promise {
		future_to_promise(async move {
			Ok(JsValue::NULL)
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
							//let js_array = Array::new_with_length(photos.len() as u32);
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