use serde::{Deserialize,Serialize};
use upholi_lib::{EncryptedData, EncryptedKeyInfo, ShareType};
use upholi_lib::http::request::CreateAlbum;
use upholi_lib::http::response::PhotoMinimal;
use upholi_lib::{KeyInfo, http::response};
use upholi_lib::result::Result;
use crate::encryption;
use crate::encryption::symmetric::decrypt_data_base64;

use super::Entity;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ShareData {
	Album(AlbumShareData)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AlbumShareData {
	pub album_id: String,
	pub album_key: KeyInfo,
	pub photo_keys: Vec<KeyInfo>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsShare {
	pub id: String,
	pub type_: ShareType,
}

pub struct Share {
	encrypted: response::Share,
	data: ShareData,
	js_value: JsShare
}

impl Entity for Share {
	type TEncrypted = response::Share;
	type TData = ShareData;
	type TJavaScript = JsShare;

	fn from_encrypted(source: Self::TEncrypted, _key_name: &str, key: &[u8]) -> Result<Self> {
		let data_json = decrypt_data_base64(key, &source.data)?;
		let data: ShareData = match source.type_ {
			ShareType::Album => ShareData::Album(serde_json::from_slice(&data_json)?)
		};

		let js_value = Self::TJavaScript {
			id: source.id.clone(),
			type_: source.type_.clone(),
		};

		Ok(Self {
			encrypted: source,
			data,
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