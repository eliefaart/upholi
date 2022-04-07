use crate::encryption::symmetric::decrypt_data_base64;
use crate::exif::Exif;
use serde::{Deserialize, Serialize};
use upholi_lib::{models::EncryptedPhoto, result::Result};

use super::Entity;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoData {
	/// Hash of original photo file
	pub hash: String,
	pub width: u32,
	pub height: u32,
	pub content_type: String,
	pub exif: Option<Exif>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsPhoto {
	pub id: String,
	pub hash: String,
	pub width: u32,
	pub height: u32,
	pub content_type: String,
	pub exif: Option<Exif>,
}

pub struct Photo {
	key: Vec<u8>,
	encrypted: EncryptedPhoto,
	data: PhotoData,
	js_value: JsPhoto,
}

impl Entity for Photo {
	type TEncrypted = EncryptedPhoto;
	type TData = PhotoData;
	type TJavaScript = JsPhoto;

	fn from_encrypted(source: Self::TEncrypted, key: &[u8]) -> Result<Self> {
		let photo_data_json = decrypt_data_base64(key, &source.data)?;
		let photo_data: PhotoData = serde_json::from_slice(&photo_data_json)?;

		let js_value = Self::TJavaScript {
			id: source.id.clone(),
			hash: source.hash.clone(),
			width: source.width as u32,
			height: source.height as u32,
			content_type: photo_data.content_type.clone(),
			exif: photo_data.exif.clone(),
		};

		Ok(Self {
			key: key.to_vec(),
			encrypted: source,
			data: photo_data,
			js_value,
		})
	}

	fn from_encrypted_with_owner_key(source: Self::TEncrypted, key: &[u8]) -> Result<Self> {
		let photo_key = decrypt_data_base64(key, &source.key)?;
		Self::from_encrypted(source, &photo_key)
	}

	fn get_key(&self) -> &[u8] {
		&self.key
	}

	fn get_id(&self) -> &str {
		&self.encrypted.id
	}

	fn get_data_mut(&mut self) -> &mut Self::TData {
		&mut self.data
	}

	fn get_data(&self) -> &Self::TData {
		&self.data
	}

	fn get_encrypted(&self) -> &Self::TEncrypted {
		&self.encrypted
	}

	fn as_js_value(&self) -> &Self::TJavaScript {
		&self.js_value
	}
}
