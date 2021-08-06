//extern crate hyper_multipart_rfc7578;

use std::{error::Error, ops::Deref, sync::Arc};
use exif::Exif;
use image::EncodableLayout;
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

pub trait MultipartFile {
    fn name(&self) -> String;
    fn as_bytes(&self) -> Vec<u8>;
    fn len(&self) -> usize;
}
struct MyUpload {
    pub name: String,
    pub data: Vec<u8>,
}
impl MultipartFile for MyUpload {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn as_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}


fn multipart_for_binary<T>(items: &[T]) -> (Vec<u8>, usize) where T: MultipartFile {
	// Based on:
	// https://www.reddit.com/r/rust/comments/69ywsr/multipartform_request_with_reqwest/dhaqszf?utm_source=share&utm_medium=web2x&context=3

	let mut body = Vec::new();
	let rn = b"\r\n";
	let body_boundary = br"--MULTIPARTBINARY";
	let end_boundary =  br"--MULTIPARTBINARY--";
	let enc = br"Content-Transfer-Encoding: binary";

	let field_name = match items.len() {
		1 => "file",
		_ => "files",
	};

	body.extend(rn);
	body.extend(rn);

	for item in items {
		let name = item.name();
		let data = item.as_bytes();
		let disp = format!("Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"", field_name, name);
		let content_type = br"Content-Type: application/octet-stream";

		body.extend(body_boundary.as_ref());
		body.extend(rn);
		body.extend(disp.as_bytes());
		body.extend(rn);
		body.extend(content_type.as_ref());
		body.extend(rn);
		body.extend(enc.as_ref());
		body.extend(rn);
		body.extend(rn);
		body.extend(data.as_slice());
		body.extend(rn);
	}
	body.extend(end_boundary.as_ref());
	body.extend(rn);
	body.extend(rn);

	let content_length = body.len();

	(body, content_length)
}

struct UpholiClientInternalHelper { }

impl UpholiClientInternalHelper {
	pub async fn upload_photo(base_url: &str, private_key: &[u8], image: &PhotoUploadInfo) -> Result<()> {

		let form = multipart::Form::new()
			// Adding just a simple text field...
			.text("username", "seanmonstar");
			// And a file...
			//.file("photo", "/path/to/photo.png")?;

		// Customize all the details of a Part if needed...
		let bio = multipart::Part::text("hallo peeps")
			.file_name("bio.txt")
			.mime_str("text/plain")?;

		// Add the custom part to our form...
		let form = form.part("biography", bio);

		// And finally, send the form
		let client = reqwest::Client::new();
		let url = format!("{}/api/photo/{}/thumbnail", &base_url, "avc").to_owned();
		let resp = client
			.post(url)
			.multipart(form)
			.send().await?;







		let client = reqwest::Client::new();
		let mut request_data = UpholiClient::get_upload_photo_request_data(&image, &private_key)?;

		// Decrypt photo key
		let photo_key = encryption::decrypt_data(private_key, &request_data.key)?;

		// Encrypt photo bytes
		let thumbnail_encrypted = encryption::encrypt_slice(&photo_key, &image.bytes_thumbnail())?;
		let preview_encrypted = encryption::encrypt_slice(&photo_key, &image.bytes_preview())?;
		let original_encrypted = encryption::encrypt_slice(&photo_key, &image.bytes_original())?;

		// Store nonces in request data
		request_data.thumbnail_nonce = thumbnail_encrypted.nonce;
		request_data.preview_nonce = preview_encrypted.nonce;
		request_data.original_nonce = original_encrypted.nonce;

		// Create photo
		let url = format!("{}/api/photo", &base_url).to_owned();
		let response = client.post(&url).json(&request_data).send().await?;
		let photo: response::UploadPhoto = response.json().await?;


		// Upload photo bytes
		// !!!! Ok, try hyper or something?
		// I feel text()
		//let form = reqwest::multipart::Form::new()
			// This sends way too many bytes I think?
			//.part("metadata", multipart::Part::text("json of photo data"))
			//.part("thumbnail", multipart::Part::bytes(base64::decode_config(thumbnail_encrypted.base64, base64::STANDARD)?))
			//.part("thumbnail", multipart::Part::bytes(thumbnail_encrypted.base64.as_bytes()))
			// .part("original", multipart::Part::bytes(image.bytes())) //.file_name("thumbnail").mime_str("image/jpg"));
			//;

		let url = format!("{}/api/photo/{}/thumbnail", &base_url, &photo.id).to_owned();

		let multipart_file = MyUpload {
			name: "testy".into(),
			data: thumbnail_encrypted.base64.as_bytes().into()
		};
		let (multipart_body, content_length) = multipart_for_binary(&vec!{multipart_file});
		//client.put(&url).body(thumbnail_encrypted.base64).send().await?;
		client.put(&url)
			.body(multipart_body)
			.header("Content-Type", format!("multipart/form-data; boundary=MULTIPARTBINARY"))
			.header("Content-Length", content_length)
			.send().await?;
		//client.put(&url).multipart(form).send().await?;
		let url = format!("{}/api/photo/{}/preview", &base_url, &photo.id).to_owned();
		client.put(&url).body(preview_encrypted.base64).send().await?;
		let url = format!("{}/api/photo/{}/original", &base_url, &photo.id).to_owned();
		client.put(&url).body(original_encrypted.base64).send().await?;

		Ok(())
	}

	pub async fn get_photos(base_url: &str) -> Result<Vec<response::PhotoMinimal>> {
		let url = format!("{}/api/photos", &base_url).to_owned();

		let response = reqwest::get(url).await?;
		let photos = response.json::<Vec<response::PhotoMinimal>>().await?;
		Ok(photos)
	}

	pub async fn get_photo_base64(base_url: &str, private_key: &[u8], id: &str) -> Result<String> {
		let url = format!("{}/api/photo/{}/thumbnail", base_url, id);

		// Get photo bytes
		let response = reqwest::get(url).await?;
		let photo_base64_bytes = response.bytes().await?;
		let photo_base64 = String::from_utf8(photo_base64_bytes.to_vec())?;

		// Decrypt photo bytes
		let photo = Self::get_photo_info(base_url, id).await?;
		let photo_key = encryption::decrypt_data(private_key, &photo.key)?;
		let bytes = encryption::decrypt_base64(&photo_key, &photo.thumbnail_nonce.as_bytes(), &photo_base64)?;

		Ok(base64::encode_config(&bytes, base64::STANDARD))
	}

	/// Get photo as returned by server.
	pub async fn get_photo_info(base_url: &str, id: &str) -> Result<request::UploadPhoto> {
		let url = format!("{}/api/photo/{}", base_url, id);
		let response = reqwest::get(url).await?;
		let encrypted_photo = response.json::<request::UploadPhoto>().await?;
		Ok(encrypted_photo)
	}

	pub async fn get_photo_data(base_url: &str, private_key: &[u8], id: &str) -> Result<PhotoData> {
		let photo = Self::get_photo_info(base_url, id).await?;

		let decypted_key = decrypt(private_key, photo.key.nonce.as_bytes(), photo.key.base64.as_bytes())?;
		let decrypted_data_json = decrypt(&decypted_key, photo.data.nonce.as_bytes(), photo.data.base64.as_bytes())?;
		let decrypted_data_json = String::from_utf8(decrypted_data_json)?;

		let data = serde_json::from_str::<PhotoData>(&decrypted_data_json)?;

		Ok(data)
	}

	pub async fn delete_photo(base_url: &str, id: &str) -> Result<()> {
		let url = format!("{}/api/photo/{}", base_url, id);
		let client = reqwest::Client::new();
		client.delete(url).send().await?;
		Ok(())
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
			match UpholiClientInternalHelper::upload_photo(&base_url, &private_key, &image).await {
				Ok(_) => Ok(JsValue::NULL),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

	#[wasm_bindgen(js_name = getPhotos)]
	pub fn get_photos(&self) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientInternalHelper::get_photos(&base_url).await {
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

	#[wasm_bindgen(js_name = deletePhoto)]
	pub fn delete_photo(&self, id: String) -> js_sys::Promise {
		let base_url = self.base_url.to_owned();

		future_to_promise(async move {
			match UpholiClientInternalHelper::delete_photo(&base_url,&id).await {
				Ok(_) => Ok(JsValue::UNDEFINED),
				Err(error) => Err(String::from(format!("{}", error)).into())
			}
		})
	}

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
			share_keys: vec!{},
			thumbnail_nonce: String::new(),
			preview_nonce: String::new(),
			original_nonce: String::new()
		})
	}
}