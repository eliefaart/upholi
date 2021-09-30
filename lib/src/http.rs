pub mod request {
	use serde::{Deserialize, Serialize};

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UploadPhoto {
		pub hash: String,
		pub width: u32,
		pub height: u32,
		/// Encrypted data, contains width, height, exif, etc
		pub data: crate::EncryptedData,
		pub key: crate::EncryptedData,
		pub key_hash: String,
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
		pub data: crate::EncryptedData,
		pub key: crate::EncryptedData,
		pub key_hash: String
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct CreateShare {
		pub type_: crate::ShareType,
		pub password: crate::EncryptedData,
		pub data: crate::EncryptedData,
		pub key: crate::EncryptedData,
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
		pub password: String,
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct EntityAuthorizationProof {
		/// Hash of the private key of an entity
		pub key_hash: String,
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
		pub data: crate::EncryptedData,
		pub key: crate::EncryptedData,
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

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct CreateShare {
		pub id: String
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct Share {
		pub id: String,
		pub user_id: String,
		pub type_: crate::ShareType,
		pub password: crate::EncryptedData,
		pub key: crate::EncryptedData,
		pub data: crate::EncryptedData,
	}
}