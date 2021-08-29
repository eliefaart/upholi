use serde::{Deserialize,Serialize};
use upholi_lib::http::response::PhotoMinimal;
use upholi_lib::{ShareKey, http::response};
use upholi_lib::result::Result;
use crate::encryption::symmetric::decrypt_data_base64;

use super::Entity;

pub struct Album {
	decrypted: DecryptedAlbum,
	js_value: JsAlbum
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumData {
	pub title: String,
	pub tags: Vec<String>,
	pub photos: Vec<String>,
	pub thumbnail_photo_id: Option<String>,
}

pub struct DecryptedAlbum {
	pub id: String,
	pub user_id: String,
	pub data: AlbumData,
	pub key: ShareKey,
	pub share_keys: Vec<ShareKey>
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

impl Entity for Album {
	type TEncrypted = response::Album;
	type TDecrypted = DecryptedAlbum;
	type TData = AlbumData;
	type TJavaScript = JsAlbum;

	fn from_encrypted(source: Self::TEncrypted, private_key: &[u8]) -> Result<Self> {
		let key = decrypt_data_base64(private_key, &source.key)?;
		let album_data_json = decrypt_data_base64(&key, &source.data)?;
		let album_data: AlbumData = serde_json::from_slice(&album_data_json)?;

		let js_value = Self::TJavaScript {
			id: source.id.clone(),
			title: album_data.title.clone(),
			tags: album_data.tags.clone(),
			photos: album_data.photos.clone(),
			thumbnail_photo_id: album_data.thumbnail_photo_id.clone(),
		};

		let decrypted = DecryptedAlbum {
			id: source.id,
			user_id: source.user_id,
			data: album_data,
			key: ShareKey {
				id: String::new(),
				key: String::new()
			},
			share_keys: vec!{}
		};

		Ok(Self {
			decrypted,
			js_value
		})
	}

	fn get_id(&self) -> &str {
		&self.decrypted.id
	}

	fn get_data(&self) -> &Self::TData {
		&self.decrypted.data
	}

	fn into_js_value(&self) -> &Self::TJavaScript {
		&self.js_value
	}
}