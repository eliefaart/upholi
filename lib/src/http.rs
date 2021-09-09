pub mod request {
	use serde::{Deserialize, Serialize};

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UploadPhoto {
		pub hash: String,
		pub width: u32,
		pub height: u32,
		/// Key that all data and file bytes of this photo is encrypted with. This key is encrypted with the owner's private key.
		pub key: crate::EncryptedData,
		/// Encrypted data, contains width, height, exif, etc
		pub data: crate::EncryptedData,
		pub share_keys: Vec<crate::EncryptedShareKey>,
		pub thumbnail_nonce: String,
		pub preview_nonce: String,
		pub original_nonce: String
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct CheckPhotoExists {
		pub hash: String
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct CreateAlbum {
		pub key: crate::EncryptedData,
		pub data: crate::EncryptedData,
		pub share_keys: Vec<crate::EncryptedShareKey>
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct Register {
		pub username: String,
		pub password: String,
		pub key: crate::EncryptedData,
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct Login {
		pub username: String,
		/// Username encrypted with user's private key
		pub password: String,
	}
}

pub mod response {
	use serde::{Deserialize, Serialize};

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UserInfo {
		pub id: String,
		pub username: String,
		pub key: crate::EncryptedData
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UploadPhoto {
		pub id: String
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct CreateAlbum {
		pub id: String
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct Photo {
		pub id: String,
		pub user_id: String,
		pub hash: String,
		pub width: i32,
		pub height: i32,
		pub data: crate::EncryptedData,
		pub key: crate::EncryptedData,
		pub share_keys: Vec<crate::EncryptedShareKey>,
		pub thumbnail_nonce: String,
		pub preview_nonce: String,
		pub original_nonce: String
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct PhotoMinimal {
		pub id: String,
		pub width: u32,
		pub height: u32
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct Album {
		pub id: String,
		pub user_id: String,
		pub key: crate::EncryptedData,
		pub data: crate::EncryptedData,
		pub share_keys: Vec<crate::EncryptedShareKey>
	}

	impl Clone for PhotoMinimal {
		fn clone(&self) -> Self {
			Self {
				id: self.id.clone(),
				height: self.height,
				width: self.width
			}
		}
	}
}