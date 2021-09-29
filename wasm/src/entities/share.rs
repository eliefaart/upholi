use serde::{Deserialize,Serialize};
use upholi_lib::ShareType;
use upholi_lib::http::response;
use upholi_lib::result::Result;
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
	pub album_key: Vec<u8>,
	pub photo_keys: Vec<Vec<u8>>
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsShare {
	pub id: String,
	pub type_: ShareType,
	pub password: String
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

	fn from_encrypted(source: Self::TEncrypted, key: &[u8]) -> Result<Self> {
		let data_json = decrypt_data_base64(key, &source.data)?;
		let data: ShareData = serde_json::from_slice(&data_json)?;

		let password = decrypt_data_base64(key, &source.password)?;

		let js_value = Self::TJavaScript {
			id: source.id.clone(),
			type_: source.type_.clone(),
			password: String::from_utf8(password)?
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