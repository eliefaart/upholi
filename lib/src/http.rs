use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShareKey {
	id: String,
	key: EncryptedData
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedData {
	pub nonce: String,
	pub base64: String
}

impl Clone for EncryptedData {
	fn clone(&self) -> Self {
		Self {
			base64: self.base64.clone(),
			nonce: self.nonce.clone()
		}
	}
}

pub mod request {
	use serde::{Deserialize, Serialize};

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UploadPhoto {
		pub width: u32,
		pub height: u32,
		/// Encrypted data, contains width, height, exif, etc
		pub data: super::EncryptedData,
		pub data_version: u32,
		/// Key that all data and file bytes of this photo is encrypted with. This key is encrypted with the owner's private key.
		pub key: super::EncryptedData,
		pub share_keys: Vec<super::ShareKey>
	}
}

pub mod response {
	use serde::{Deserialize, Serialize};

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UploadPhoto {
		pub id: String
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct PhotoMinimal {
		id: String,
		width: u32,
		height: u32
	}
}