use std::error::Error;
use image::{GenericImageView, ImageFormat};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod error;
mod aes256;
mod images;

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
pub struct Image {
	image: images::Image,
	exif: String
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Image {
		let exif_orientation = 1;

		let slice = &bytes[0..];
		let image = images::Image::from_buffer(slice, exif_orientation).unwrap();

        Image {
            image,
			exif: "".to_string()
        }
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