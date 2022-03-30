use crate::{encryption::symmetric::decrypt_data_base64, hashing};
use serde::{Deserialize, Serialize};
use upholi_lib::models::EncryptedShare;
use upholi_lib::result::Result;
use upholi_lib::ShareType;

use super::{Entity, EntityKey};

pub struct Share {
	key: Vec<u8>,
	encrypted: EncryptedShare,
	data: ShareData,
	js_value: JsShare,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ShareData {
	Album(AlbumShareData),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumShareData {
	pub album_id: String,
	pub album_key: Vec<u8>,
	pub photos: Vec<EntityKey>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsShare {
	pub id: String,
	pub identifier_hash: String,
	pub type_: ShareType,
	pub password: String,
	/// Fine, while all shares represent albums.
	pub data: JsShareData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum JsShareData {
	Album(JsAlbumShareData),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JsAlbumShareData {
	pub album_id: String,
}

impl From<&ShareData> for JsShareData {
	fn from(data: &ShareData) -> Self {
		match data {
			ShareData::Album(album_data) => JsShareData::Album(JsAlbumShareData {
				album_id: album_data.album_id.clone(),
			}),
		}
	}
}

impl Share {
	pub fn get_identifier_hash(type_: &ShareType, type_id: &str) -> Result<String> {
		let identifier_string = match type_ {
			ShareType::Album => {
				format!("album:{}", type_id)
			}
		};

		hashing::compute_sha256_hash(identifier_string.as_bytes())
	}
}

impl Entity for Share {
	type TEncrypted = EncryptedShare;
	type TData = ShareData;
	type TJavaScript = JsShare;

	fn from_encrypted(source: Self::TEncrypted, key: &[u8]) -> Result<Self> {
		let data_json = decrypt_data_base64(key, &source.data)?;
		let data: ShareData = serde_json::from_slice(&data_json)?;

		let password = decrypt_data_base64(key, &source.password)?;

		let js_value = Self::TJavaScript {
			id: source.id.clone(),
			type_: source.type_.clone(),
			identifier_hash: source.identifier_hash.clone(),
			password: String::from_utf8(password)?,
			data: (&data).into(),
		};

		Ok(Self {
			key: key.to_vec(),
			encrypted: source,
			data,
			js_value,
		})
	}

	fn from_encrypted_with_owner_key(source: Self::TEncrypted, key: &[u8]) -> Result<Self> {
		let share_key = decrypt_data_base64(key, &source.key)?;
		Self::from_encrypted(source, &share_key)
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
