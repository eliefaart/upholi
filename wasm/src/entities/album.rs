use serde::{Deserialize,Serialize};
use upholi_lib::{EncryptedKeyInfo, KeyInfo};
use upholi_lib::http::request::CreateAlbum;
use upholi_lib::http::response::PhotoMinimal;
use upholi_lib::http::response;
use upholi_lib::result::Result;
use crate::encryption::symmetric::decrypt_data_base64;
use crate::hashing::compute_sha256_hash;

use super::share::{AlbumShareData, ShareData};
use super::{Entity, Shareable};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumData {
	pub title: String,
	pub tags: Vec<String>,
	pub photos: Vec<String>,
	pub thumbnail_photo_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsAlbum {
	pub id: String,
	pub title: String,
	pub tags: Vec<String>,
	pub photos: Vec<String>,
	pub thumbnail_photo_id: Option<String>,
}

/// Album, but with enriched photo data
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumDetailed {
	pub id: String,
	pub title: String,
	pub tags: Vec<String>,
	pub photos: Vec<PhotoMinimal>,
	pub thumbnail_photo: Option<PhotoMinimal>,
}

pub struct Album {
	private_key: Vec<u8>,
	encrypted: response::Album,
	data: AlbumData,
	keys: Vec<EncryptedKeyInfo>,
	js_value: JsAlbum
}

impl Album {
	pub fn create_update_request_struct(&self) -> Result<CreateAlbum> {
		let owner_key = self.keys.iter().find(|key| key.name == crate::OWNER_KEY_NAME).ok_or("Owner key not found")?;
		let album_key = crate::encryption::symmetric::decrypt_data_base64(&self.private_key, &owner_key.encrypted_key)?;

		let data_json = serde_json::to_string(&self.data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&album_key, data_bytes)?;

		Ok(CreateAlbum {
			data: data_encrypt_result.into(),
			keys: self.keys.clone(),
			key_hash: compute_sha256_hash(&album_key)?
		})
	}
}

impl Entity for Album {
	type TEncrypted = response::Album;
	type TData = AlbumData;
	type TJavaScript = JsAlbum;

	fn from_encrypted(source: Self::TEncrypted, key_name: &str, key: &[u8]) -> Result<Self> {
		let owner_key = source.keys.iter().find(|key| key.name == key_name).ok_or(format!("Key with name {} not found", key_name))?;
		let album_key = decrypt_data_base64(key, &owner_key.encrypted_key)?;
		let album_data_json = decrypt_data_base64(&album_key, &source.data)?;
		let album_data: AlbumData = serde_json::from_slice(&album_data_json)?;

		let js_value = Self::TJavaScript {
			id: source.id.clone(),
			title: album_data.title.clone(),
			tags: album_data.tags.clone(),
			photos: album_data.photos.clone(),
			thumbnail_photo_id: album_data.thumbnail_photo_id.clone(),
		};

		let keys = source.keys.clone();

		Ok(Self {
			private_key: key.to_vec(),
			encrypted: source,
			data: album_data,
			keys,
			js_value
		})
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

	fn as_js_value(&self) -> &Self::TJavaScript {
		&self.js_value
	}
}

impl Shareable for Album {
	fn create_share_data(&self, key: &[u8], password: &str) -> Result<ShareData> {
		let album_key = self.keys.iter().find(|key| key.name == crate::OWNER_KEY_NAME).ok_or("Owner key not found")?;
		let album_key = decrypt_data_base64(key, &album_key.encrypted_key)?;

		// How is this function going to figure out the photo's keys?
		// It has the photo IDs

		Ok(ShareData::Album(AlbumShareData {
			share_password: password.into(),
			album_id: self.get_id().into(),
			album_key: KeyInfo::from_bytes("", &album_key),
			photo_keys: vec!{}
		}))
	}
}