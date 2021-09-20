use serde::{Deserialize,Serialize};
use upholi_lib::http::response::PhotoMinimal;
use upholi_lib::{KeyInfo, http::response};
use upholi_lib::result::Result;
use crate::encryption;
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
	pub keys: Vec<KeyInfo>
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

impl Album {
	pub fn update_share_options(&self, shared: bool, password: &str) -> Result<()> {
		let key_name = format!("album:{}", &self.get_id());
		let key = encryption::symmetric::derive_key_from_string(password, self.get_id())?;


		// Ok...
		// Photo. is encrypted with key k1 and nonce n1.
		// Album. is encrypted with k2 and nonce n2.
		//
		// photo has shares array;
		// - { id: "user:abc123",  data: { bytes: [enc.], nonce: sn1 } },		encrypted using user's key
		// - { id: "album:xyz098", data: { bytes: [enc.], nonce: sn2 } },		encrypted using album's key
		// 'enc.' = encrypted photo key bytes

		//encrypted_share.

		Ok(())
	}
}

impl Entity for Album {
	type TEncrypted = response::Album;
	type TDecrypted = DecryptedAlbum;
	type TData = AlbumData;
	type TJavaScript = JsAlbum;

	fn from_encrypted(source: Self::TEncrypted, private_key: &[u8]) -> Result<Self> {
		let owner_key = source.keys.iter().find(|key| key.name == crate::OWNER_KEY_NAME).ok_or("Owner key not found")?;
		let key = decrypt_data_base64(private_key, &owner_key.encrypted_key)?;
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

	fn get_data(&self) -> &Self::TData {
		&self.decrypted.data
	}

	fn as_js_value(&self) -> &Self::TJavaScript {
		&self.js_value
	}
}