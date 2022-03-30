use crate::encryption::symmetric::decrypt_data_base64;
use crate::entities::EntityKey;
use crate::hashing::compute_sha256_hash;
use serde::{Deserialize, Serialize};
use upholi_lib::models::{EncryptedAlbum, EncryptedAlbumUpsert};
use upholi_lib::result::Result;

use super::photo::Photo;
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
pub struct JsAlbumFull {
	pub id: String,
	pub title: String,
	pub tags: Vec<String>,
	pub photos: Vec<JsAlbumPhoto>,
	pub thumbnail_photo: Option<JsAlbumPhoto>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsAlbumPhoto {
	pub id: String,
	pub width: u32,
	pub height: u32,
	pub key: Option<String>,
}

pub struct Album {
	key: Vec<u8>,
	encrypted: EncryptedAlbum,
	data: AlbumData,
	js_value: JsAlbum,
}

impl Album {
	pub fn create_update_request_struct(&self) -> Result<EncryptedAlbumUpsert> {
		let data_json = serde_json::to_string(&self.data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&self.key, data_bytes)?;

		Ok(EncryptedAlbumUpsert {
			data: data_encrypt_result.into(),
			key: self.encrypted.key.clone(),
			key_hash: compute_sha256_hash(&self.key)?,
		})
	}
}

impl Entity for Album {
	type TEncrypted = EncryptedAlbum;
	type TData = AlbumData;
	type TJavaScript = JsAlbum;

	fn from_encrypted(source: Self::TEncrypted, key: &[u8]) -> Result<Self> {
		let album_data_json = decrypt_data_base64(key, &source.data)?;
		let album_data: AlbumData = serde_json::from_slice(&album_data_json)?;

		let js_value = Self::TJavaScript {
			id: source.id.clone(),
			title: album_data.title.clone(),
			tags: album_data.tags.clone(),
			photos: album_data.photos.clone(),
			thumbnail_photo_id: album_data.thumbnail_photo_id.clone(),
		};

		Ok(Self {
			key: key.to_vec(),
			encrypted: source,
			data: album_data,
			js_value,
		})
	}

	fn from_encrypted_with_owner_key(source: Self::TEncrypted, key: &[u8]) -> Result<Self> {
		let album_key = decrypt_data_base64(key, &source.key)?;
		Self::from_encrypted(source, &album_key)
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

	fn as_js_value(&self) -> &Self::TJavaScript {
		&self.js_value
	}
}

impl Shareable for Album {
	fn create_share_data(&self, key: &[u8], photos: &[Photo]) -> Result<ShareData> {
		let album_key = decrypt_data_base64(key, &self.encrypted.key)?;

		// How is this function going to figure out the photo's keys?
		// It has the photo IDs
		let mut photos_info = vec![];
		for photo_id in &self.data.photos {
			let photo = photos.iter().find(|p| p.get_id() == photo_id);
			if let Some(photo) = photo {
				let entity = EntityKey {
					id: photo_id.clone(),
					key: base64::encode_config(photo.get_key(), base64::STANDARD),
				};
				photos_info.push(entity);
			}
		}

		Ok(ShareData::Album(AlbumShareData {
			album_id: self.get_id().into(),
			album_key,
			photos: photos_info,
		}))
	}
}
