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
	pub base64: String,
	/// Version of format of data that was encrypted.
	/// For future use.
	pub format_version: i32
}

impl Clone for EncryptedData {
	fn clone(&self) -> Self {
		Self {
			base64: self.base64.clone(),
			nonce: self.nonce.clone(),
			format_version: self.format_version
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
		/// Key that all data and file bytes of this photo is encrypted with. This key is encrypted with the owner's private key.
		pub key: super::EncryptedData,
		/// Encrypted data, contains width, height, exif, etc
		pub data: super::EncryptedData,
		pub share_keys: Vec<super::ShareKey>,
		pub thumbnail_nonce: String,
		pub preview_nonce: String,
		pub original_nonce: String
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
	pub struct Photo {
		pub id: String,
		pub user_id: String,
		pub width: i32,
		pub height: i32,
		pub data: super::EncryptedData,
		pub key: super::EncryptedData,
		pub share_keys: Vec<super::ShareKey>,
		pub thumbnail_nonce: String,
		pub preview_nonce: String,
		pub original_nonce: String
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct PhotoMinimal {
		id: String,
		width: u32,
		height: u32
	}
}