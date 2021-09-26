use serde::{Deserialize,Serialize};
use upholi_lib::{KeyInfo, http::response};
use upholi_lib::result::Result;
use crate::encryption::symmetric::decrypt_data_base64;
use crate::exif::Exif;

use super::Entity;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PhotoData {
	/// Hash of original photo file
	pub hash: String,
	pub width: u32,
	pub height: u32,
	pub content_type: String,
	pub exif: Exif
}

pub struct DecryptedPhoto {
	pub id: String,
	pub user_id: String,
	pub hash: String,
	pub width: i32,
	pub height: i32,
	pub data: PhotoData,
	pub keys: Vec<KeyInfo>,
	pub thumbnail_nonce: String,
	pub preview_nonce: String,
	pub original_nonce: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsPhoto {
	pub id: String,
	pub hash: String,
	pub width: u32,
	pub height: u32,
	pub content_type: String,
	pub exif: Exif
}

pub struct Photo {
	decrypted: DecryptedPhoto,
	js_value: JsPhoto
}

impl Entity for Photo {
	type TEncrypted = response::Photo;
	type TDecrypted = DecryptedPhoto;
	type TData = PhotoData;
	type TJavaScript = JsPhoto;

	fn from_encrypted(source: Self::TEncrypted, key_name: &str, key: &[u8]) -> Result<Self> {
		let owner_key = source.keys.iter().find(|key| key.name == key_name).ok_or(format!("Key with name {} not found", key_name))?;
		let key = decrypt_data_base64(key, &owner_key.encrypted_key)?;
		let photo_data_json = decrypt_data_base64(&key, &source.data)?;
		let photo_data: PhotoData = serde_json::from_slice(&photo_data_json)?;

		let js_value = Self::TJavaScript {
			id: source.id.clone(),
			hash: source.hash.clone(),
			width: source.width as u32,
			height: source.height as u32,
			content_type: photo_data.content_type.clone(),
			exif: photo_data.exif.clone()
		};

		let decrypted = DecryptedPhoto {
			id: source.id.clone(),
			user_id: source.user_id,
			hash: source.hash,
			width: source.width,
			height: source.height,
			thumbnail_nonce: source.thumbnail_nonce,
			preview_nonce: source.preview_nonce,
			original_nonce: source.original_nonce,
			data: photo_data,
			keys: vec!{}
		};

		Ok(Self {
			decrypted,
			js_value
		})
	}

	fn get_id(&self) -> &str {
		&self.decrypted.id
	}

	fn get_data_mut(&mut self) -> &mut Self::TData {
		&mut self.decrypted.data
	}

	fn get_decrypted(&self) -> &Self::TDecrypted {
		&self.decrypted
	}

	fn get_data(&self) -> &Self::TData {
		&self.decrypted.data
	}

	fn as_js_value(&self) -> &Self::TJavaScript {
		&self.js_value
	}
}