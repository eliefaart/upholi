use std::error::Error;
use exif::Exif;
use image::{GenericImageView, ImageFormat};
use wasm_bindgen::{prelude::*};
use uuid::Uuid;

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

//pub fn example(input: &str) -> String {}
//pub async fn example_async() -> String {}

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
	//pub bytes: js_sys::Uint8Array,
	//pub bytes: Vec<u8>,
	exif: exif::Exif
	//exif: Exif
}
#[wasm_bindgen]
pub struct UpholiExif {
	exif: exif::Exif
}

#[wasm_bindgen]
impl UpholiExif{
	pub fn get_manufactorer(&mut self) -> js_sys::JsString {
        String::from("").into()
    }
}

// impl wasm_bindgen::convert::IntoWasmAbi for Option<String> {
//     type Abi = JsValue;

//     fn into_abi(self) -> Self::Abi {
//         JsValue::from_serde(&self).unwrap().into_abi()
//     }
// }
// #[wasm_bindgen]
// pub struct UpholiExif {
// 	pub manufactorer: js_sys::JsString,
// 	pub model: Option<js_sys::JsString>,
// 	pub aperture: Option<js_sys::JsString>,
// 	pub exposure_time: Option<js_sys::JsString>,
// 	pub iso: Option<i32>,
// 	pub focal_length: Option<i32>,
// 	pub focal_length_35mm_equiv: Option<i32>,
// 	pub orientation: Option<i32>,
// 	pub date_taken: Option<js_sys::Date>,
// 	pub gps_latitude: Option<f32>,
// 	pub gps_longitude: Option<f32>
// }

// #[wasm_bindgen]
// pub struct Exif {
// 	manufactorer: Option<String>,
// 	pub iso: Option<i32>,
// 	#[wasm_bindgen(js_name = focalLength)]
// 	pub focal_length: Option<i32>,
// 	#[wasm_bindgen(js_name = focalLength35mmEquiv)]
// 	pub focal_length_35mm_equiv: Option<i32>,
// }

// impl Copy for Exif { }
// impl Clone for Exif {
//     fn clone(&self) -> Exif {
//         Exif {
// 			manufactorer: self.manufactorer.clone(),
// 			iso: self.iso,
// 			focal_length: self.focal_length,
// 			focal_length_35mm_equiv: self.focal_length_35mm_equiv
// 		}
//     }
// }


#[wasm_bindgen]
impl ImageUploadInfo {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> ImageUploadInfo {
		let exif = exif::Exif::parse_from_photo_bytes(bytes);
		match exif {
			Ok(exif) => {
				let exif_orientation = exif.orientation.unwrap_or(1);

				//let slice = &bytes[0..];
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
        self.image.bytes_original[0..20].to_vec()
    }

	#[wasm_bindgen(getter, js_name = exifFocalLength)]
    pub fn exif_focal_length(&self) -> Option<i32> {
        self.exif.focal_length
    }

	#[wasm_bindgen(getter, js_name = exifManufactorer)]
    pub fn exif_manufacturer(&self) -> Option<String> {
        self.exif.manufactorer.to_owned()
    }

	pub fn get_bytes(&mut self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(&self.image.bytes_original[..])
    }

	pub fn get_preview_bytes(&mut self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(&self.image.bytes_preview[..])
    }

	pub fn get_thumbnail_bytes(&mut self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(&self.image.bytes_thumbnail[..])
    }
}









// #[wasm_bindgen]
// pub struct UpholiPhotoMinimal {
// 	id: String,
// 	pub width: u32,
// 	pub height: u32
// }

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

// #[wasm_bindgen]
// pub struct UpholiClient {
// 	base_url: String
// }

// #[wasm_bindgen]
// impl UpholiClient {
// 	#[wasm_bindgen(constructor)]
//     pub fn new(base_url: String) -> UpholiClient {
// 		UpholiClient {
// 			base_url
// 		}
// 	}

// 	pub fn get_photos(&mut self) -> Vec<UpholiPhotoMinimal> {
// 		vec!{
// 			UpholiPhotoMinimal {
// 				id: "1".to_string(),
// 				height: 20,
// 				width: 10
// 			}
// 		}
// 	}

// 	pub fn get_array(&self) -> Vec<JsValue> {
// 		let photos: Vec<UpholiPhotoMinimal> = vec!{};
// 		photos.iter().map(JsValue::from).collect()
// 	}

// 	// pub async fn get_photo(&mut self, id: String) {}
// 	// pub async fn get_photo_bytes_original(&mut self, id: String) {}
// 	// pub async fn get_photo_bytes_preview(&mut self, id: String) {}
// 	// pub async fn get_photo_bytes_thumbnail(&mut self, id: String) {}

// 	// pub async fn get_albums(&mut self) {}
// 	// pub async fn get_album(&mut self, id: String) {}
// 	// pub async fn insert_album(&mut self) {}
// 	// pub async fn update_album(&mut self) {}
// }