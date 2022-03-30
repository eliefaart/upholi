/// API HTTP request models
pub mod request {
	use serde::{Deserialize, Serialize};

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct FindEntity {
		pub id: String,
		pub key_hash: Option<String>,
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct CheckPhotoExists {
		pub hash: String,
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
	pub struct EntityAuthorizationProof {
		/// Hash of the private key of an entity
		pub key_hash: String,
	}

	#[derive(Deserialize, Serialize, Debug)]
	pub struct FindSharesFilter {
		pub identifier_hash: Option<String>,
	}
}

/// API HTTP response models
pub mod response {
	use serde::{Deserialize, Serialize};

	/// Response data for HTTP 201 results
	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct CreatedResult {
		pub id: String,
	}

	/// Response data for HTTP 4xx & 5xx results
	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct ErrorResult {
		pub message: String,
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UserInfo {
		pub id: String,
		pub username: String,
		pub key: crate::EncryptedData,
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UploadPhoto {
		pub id: String,
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct CreateAlbum {
		pub id: String,
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct PhotoMinimal {
		pub id: String,
		pub width: u32,
		pub height: u32,
	}

	impl Clone for PhotoMinimal {
		fn clone(&self) -> Self {
			Self {
				id: self.id.clone(),
				height: self.height,
				width: self.width,
			}
		}
	}

	#[derive(Deserialize, Serialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct CreateShare {
		pub id: String,
	}
}
