use serde::{Deserialize,Serialize};
use upholi_lib::{EncryptedData, EncryptedKeyInfo};
use upholi_lib::http::request::CreateAlbum;
use upholi_lib::http::response::PhotoMinimal;
use upholi_lib::{KeyInfo, http::response};
use upholi_lib::result::Result;
use crate::encryption;
use crate::encryption::symmetric::decrypt_data_base64;

use super::Entity;

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
	decrypted: DecryptedAlbum,
	keys: Vec<EncryptedKeyInfo>,
	js_value: JsAlbum
}

impl Album {
	/// Creates/updates/removed the key for this album's public URL
	pub fn update_share_options(&mut self, shared: bool, password: &str) -> Result<()> {
		let key_name = format!("album:{}", &self.get_id());

		// Remove any existing key
		self.keys.retain(|key| key.name != key_name);

		// Then add new/updated key if shared is true
		if shared {
			// Derive encryption key from password
			let key = encryption::symmetric::derive_key_from_string(password, self.get_id())?;

			// Encrypt the album's key using the password-derived key
			let owner_key = self.keys.iter().find(|key| key.name == crate::OWNER_KEY_NAME).ok_or("Owner key not found")?;
			let album_key = crate::encryption::symmetric::decrypt_data_base64(&self.private_key, &owner_key.encrypted_key)?;
			let key_encrypt_result = crate::encryption::symmetric::encrypt_slice(&key, &album_key)?;

			let encrypted_key = EncryptedKeyInfo {
				name: key_name,
				encrypted_key: EncryptedData {
					base64: base64::encode_config(key_encrypt_result.bytes, base64::STANDARD),
					nonce: key_encrypt_result.nonce,
					format_version: 1
				}
			};

			self.keys.push(encrypted_key);
		}

		Ok(())
	}

	pub fn create_update_request_struct(&self) -> Result<CreateAlbum> {
		let owner_key = self.keys.iter().find(|key| key.name == crate::OWNER_KEY_NAME).ok_or("Owner key not found")?;
		let album_key = crate::encryption::symmetric::decrypt_data_base64(&self.private_key, &owner_key.encrypted_key)?;

		let data_json = serde_json::to_string(&self.decrypted.data)?;
		let data_bytes = data_json.as_bytes();
		let data_encrypt_result = crate::encryption::symmetric::encrypt_slice(&album_key, data_bytes)?;

		Ok(CreateAlbum {
			data: data_encrypt_result.into(),
			keys: self.keys.clone()
		})
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
			data: album_data
		};

		Ok(Self {
			private_key: private_key.to_vec(),
			decrypted,
			keys: source.keys,
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