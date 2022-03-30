use crate::hashing::compute_sha256_hash;
use reqwest::StatusCode;
use upholi_lib::http::request::{FindEntity, FindSharesFilter, Login, Register};
use upholi_lib::http::response::{CreatedResult, ErrorResult, UserInfo};
use upholi_lib::models::{
	EncryptedAlbum, EncryptedAlbumUpsert, EncryptedPhoto, EncryptedPhotoUpsert, EncryptedShare, EncryptedShareUpsert, PhotoMinimal,
};
use upholi_lib::result::Result;
use upholi_lib::PhotoVariant;

/// Client for all HTTP calls to the API.
pub struct HttpClient {
	base_url: String,
	client: reqwest::Client,
}

impl HttpClient {
	pub fn new(base_url: &str) -> Self {
		Self {
			base_url: base_url.into(),
			client: reqwest::Client::new(),
		}
	}

	pub async fn register_user(&self, body: &Register) -> Result<()> {
		let url = format!("{}/api/user/register", self.base_url).to_owned();
		let response = self.client.post(&url).json(&body).send().await?;

		if response.status() == StatusCode::OK {
			Ok(())
		} else {
			let error: ErrorResult = response.json().await?;
			Err(Box::from(error.message))
		}
	}

	pub async fn login(&self, body: &Login) -> Result<UserInfo> {
		let url = format!("{}/api/user/login", self.base_url).to_owned();
		let response = self.client.post(&url).json(&body).send().await?;

		if response.status() == StatusCode::OK {
			let user: UserInfo = response.json().await?;
			Ok(user)
		} else {
			let error: ErrorResult = response.json().await?;
			Err(Box::from(error.message))
		}
	}

	pub async fn get_user_info(&self) -> Result<UserInfo> {
		let url = format!("{}/api/user/info", self.base_url).to_owned();
		let response = self.client.get(&url).send().await?;
		let user_info: UserInfo = response.json().await?;

		Ok(user_info)
	}

	pub async fn get_photos(&self) -> Result<Vec<PhotoMinimal>> {
		let url = format!("{}/api/photos/minimal", self.base_url);
		let response = self.client.get(url).send().await?;
		let photos = response.json::<Vec<PhotoMinimal>>().await?;

		Ok(photos)
	}

	pub async fn find_photos(&self, entities: &[FindEntity]) -> Result<Vec<EncryptedPhoto>> {
		let url = format!("{}/api/photos/find", self.base_url);

		let response = self.client.post(&url).json(&entities).send().await?;
		let photos = response.json().await?;

		Ok(photos)
	}

	pub async fn find_photos_minimal(&self, entities: &[FindEntity]) -> Result<Vec<PhotoMinimal>> {
		let url = format!("{}/api/photos/find/minimal", self.base_url);

		let response = self.client.post(&url).json(&entities).send().await?;
		let photos = response.json().await?;

		Ok(photos)
	}

	pub async fn get_photo(&self, id: &str, key: &Option<String>) -> Result<EncryptedPhoto> {
		let mut url = format!("{}/api/photo/{}", self.base_url, id);
		if let Some(key) = key {
			let key_hash = compute_sha256_hash(&base64::decode_config(key, base64::STANDARD)?)?;
			url = format!("{}?key_hash={}", url, key_hash);
		}

		let response = self.client.get(url).send().await?;
		let encrypted_photo = response.json::<EncryptedPhoto>().await?;

		Ok(encrypted_photo)
	}

	//pub async fn get_photo_base64<'a>(&self, id: &str, photo_variant: PhotoVariant, key: &Option<String>) -> Result<&'a [u8]> {
	pub async fn get_photo_base64(&self, id: &str, photo_variant: &PhotoVariant, key: &Option<String>) -> Result<Vec<u8>> {
		let mut url = format!("{}/api/photo/{}/{}", self.base_url, id, photo_variant);

		if let Some(key) = key {
			let key_hash = compute_sha256_hash(&base64::decode_config(key, base64::STANDARD)?)?;
			url = format!("{}?key_hash={}", url, key_hash);
		}

		let response = self.client.get(url).send().await?;
		let bytes = response.bytes().await?.to_vec();

		Ok(bytes)
	}

	pub async fn photo_exists(&self, hash: &str) -> Result<bool> {
		let url = format!("{}/api/photo?hash={}", self.base_url, hash);

		let response = self.client.head(&url).send().await?;

		match response.status() {
			StatusCode::NO_CONTENT => Ok(true),
			StatusCode::NOT_FOUND => Ok(false),
			status_code => Err(Box::from(format!("Unexpected response code: {}", status_code))),
		}
	}

	pub async fn create_photo(
		&self,
		data: &EncryptedPhotoUpsert,
		thumbnail_bytes: &[u8],
		preview_bytes: &[u8],
		original_bytes: &[u8],
	) -> Result<String> {
		let url = format!("{}/api/photo", self.base_url).to_owned();

		// Prepare request body
		let multipart = crate::multipart::MultipartBuilder::new()
			.add_bytes("data", &serde_json::to_vec(data)?)
			.add_bytes("thumbnail", thumbnail_bytes)
			.add_bytes("preview", preview_bytes)
			.add_bytes("original", original_bytes)
			.build();

		let response = self
			.client
			.post(&url)
			.body(multipart.body)
			.header("Content-Type", multipart.content_type)
			.header("Content-Length", multipart.content_length)
			.send()
			.await?;
		let respone: CreatedResult = response.json().await?;

		Ok(respone.id)
	}

	pub async fn delete_photo(&self, id: &str) -> Result<()> {
		let url = format!("{}/api/photo/{}", self.base_url, id);
		self.client.delete(url).send().await?;

		Ok(())
	}

	pub async fn get_albums(&self) -> Result<Vec<EncryptedAlbum>> {
		let url = format!("{}/api/albums", self.base_url);
		let response = self.client.get(url).send().await?;
		let albums = response.json().await?;

		Ok(albums)
	}

	pub async fn get_album_using_key_access_proof(&self, id: &str, album_key: &[u8]) -> Result<EncryptedAlbum> {
		let album_key_hash = compute_sha256_hash(album_key)?;
		let url = format!("{}/api/album/{}?key_hash={}", self.base_url, id, &album_key_hash);
		let response = self.client.get(url).send().await?;
		let album_encrypted = response.json::<EncryptedAlbum>().await?;

		Ok(album_encrypted)
	}

	pub async fn create_album(&self, body: &EncryptedAlbumUpsert) -> Result<CreatedResult> {
		let url = format!("{}/api/album", self.base_url).to_owned();

		let request = self.client.post(&url).json(&body);
		let response = request.send().await?;
		let response_body: CreatedResult = response.json().await?;

		Ok(response_body)
	}

	pub async fn update_album(&self, id: &str, album: &EncryptedAlbumUpsert) -> Result<()> {
		let url = format!("{}/api/album/{}", self.base_url, id).to_owned();
		self.client.put(&url).json(album).send().await?;

		Ok(())
	}

	pub async fn delete_album(&self, id: &str) -> Result<()> {
		let url = format!("{}/api/album/{}", self.base_url, &id).to_owned();
		self.client.delete(&url).send().await?;

		Ok(())
	}

	pub async fn get_shares(&self, filters: Option<FindSharesFilter>) -> Result<Vec<EncryptedShare>> {
		let mut url = format!("{}/api/shares", self.base_url);
		if let Some(filters) = filters {
			if let Some(identifier_hash) = filters.identifier_hash {
				url = format!("{}?identifier_hash={}", url, identifier_hash);
			}
		}

		let response = self.client.get(url).send().await?;
		let shares = response.json().await?;

		Ok(shares)
	}

	pub async fn get_share(&self, id: &str) -> Result<EncryptedShare> {
		let url = format!("{}/api/share/{}", self.base_url, id);
		let response = self.client.get(url).send().await?;
		let share = response.json().await?;

		Ok(share)
	}

	pub async fn create_share(&self, body: &EncryptedShareUpsert) -> Result<String> {
		let url = format!("{}/api/share", self.base_url);

		let request = self.client.post(&url).json(&body);
		let response = request.send().await?;
		let response_body: CreatedResult = response.json().await?;

		Ok(response_body.id)
	}

	pub async fn update_share(&self, id: &str, body: &EncryptedShareUpsert) -> Result<()> {
		let url = format!("{}/api/share/{}", self.base_url, id);

		let request = self.client.put(&url).json(&body);
		let response = request.send().await?;
		response.json().await?;

		Ok(())
	}

	pub async fn delete_share(&self, id: &str) -> Result<()> {
		let url = format!("{}/api/share/{}", self.base_url, &id).to_owned();
		self.client.delete(&url).send().await?;

		Ok(())
	}
}
