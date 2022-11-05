use crate::models::TextItem;
use anyhow::{anyhow, Result};
use reqwest::StatusCode;
use upholi_lib::http::{
	request::{AuthenticateUserRequest, AuthorizeShareRequest, CreateUserRequest, UpsertShareRequest},
	response::GetShareResult,
};

/// Client for all HTTP calls to the API.
pub struct ApiClient {
	base_url: String,
	client: reqwest::Client,
}

pub struct File {
	pub key: String,
	pub bytes: Vec<u8>,
}

impl ApiClient {
	pub fn new(base_url: &str) -> Self {
		Self {
			base_url: base_url.into(),
			client: reqwest::Client::new(),
		}
	}

	pub async fn register(&self, body: &CreateUserRequest) -> Result<()> {
		let url = format!("{}/user", self.base_url).to_owned();
		let response = self.client.post(&url).json(&body).send().await?;

		if response.status() == StatusCode::CREATED {
			Ok(())
		} else {
			Err(anyhow!("Failed to register user"))
		}
	}

	pub async fn login(&self, username: &str, password: &str) -> Result<()> {
		let url = format!("{}/user/auth", self.base_url).to_owned();
		let body = AuthenticateUserRequest {
			username: username.into(),
			password: password.into(),
		};
		let response = self.client.post(&url).json(&body).send().await?;

		if response.status() == StatusCode::OK {
			Ok(())
		} else {
			Err(anyhow!("Login failed"))
		}
	}

	pub async fn is_authorized_for_share(&self, share_id: &str) -> Result<bool> {
		let url = format!("{}/share/{share_id}/auth", self.base_url).to_owned();
		let response = self.client.get(&url).send().await?;
		Ok(response.status() == StatusCode::OK)
	}

	pub async fn authorize_share(&self, share_id: &str, password: &str) -> Result<bool> {
		let url = format!("{}/share/{share_id}/auth", self.base_url).to_owned();
		let body = AuthorizeShareRequest { password: password.into() };
		let response = self.client.post(&url).json(&body).send().await?;

		if response.status() == StatusCode::OK {
			Ok(true)
		} else if response.status() == StatusCode::UNAUTHORIZED {
			Ok(false)
		} else {
			Err(anyhow!("Authentication failed"))
		}
	}

	pub async fn get_share(&self, id: &str) -> Result<Option<GetShareResult>> {
		let url = format!("{}/share/{id}", self.base_url).to_owned();
		let response = self.client.get(&url).send().await?;

		if response.status() == StatusCode::OK {
			Ok(Some(response.json().await?))
		} else if response.status() == StatusCode::NOT_FOUND {
			Ok(None)
		} else {
			Err(anyhow!("Failed to get share"))
		}
	}

	pub async fn list_text_keys(&self) -> Result<Vec<String>> {
		let url = format!("{}/text", self.base_url).to_owned();
		let response = self.client.get(&url).send().await?;

		if response.status() == StatusCode::OK {
			Ok(response.json().await?)
		} else {
			Err(anyhow!("Failed to get text keys"))
		}
	}

	pub async fn get_text(&self, key: &str) -> Result<Option<TextItem>> {
		let url = format!("{}/text/{key}", self.base_url).to_owned();
		let response = self.client.get(&url).send().await?;

		if response.status() == StatusCode::OK {
			Ok(Some(response.json().await?))
		} else if response.status() == StatusCode::NOT_FOUND {
			Ok(None)
		} else {
			Err(anyhow!("Failed to get text"))
		}
	}

	pub async fn set_text(&self, key: &str, body: &TextItem) -> Result<()> {
		let url = format!("{}/text/{key}", self.base_url).to_owned();
		let response = self.client.post(&url).json(&body).send().await?;

		if response.status() == StatusCode::OK {
			Ok(())
		} else {
			Err(anyhow!("Failed to set text"))
		}
	}

	pub async fn get_file(&self, key: &str) -> Result<Option<Vec<u8>>> {
		let url = format!("{}/file/{key}", self.base_url).to_owned();
		let response = self.client.get(&url).send().await?;

		if response.status() == StatusCode::OK {
			Ok(Some(response.bytes().await?.to_vec()))
		} else if response.status() == StatusCode::NOT_FOUND {
			Ok(None)
		} else {
			Err(anyhow!("Failed to get file"))
		}
	}

	pub async fn set_files(&self, files: &Vec<File>) -> Result<()> {
		let url = format!("{}/file", self.base_url).to_owned();

		// Prepare request body
		let mut multipart_builder = crate::multipart::MultipartBuilder::new();
		for file in files {
			multipart_builder = multipart_builder.add_bytes(&file.key, &file.bytes);
		}
		let multipart = multipart_builder.build();

		let response = self
			.client
			.post(&url)
			.body(multipart.body)
			.header("Content-Type", multipart.content_type)
			.header("Content-Length", multipart.content_length)
			.send()
			.await?;

		if response.status() == StatusCode::OK {
			Ok(())
		} else {
			Err(anyhow!("Failed to set text"))
		}
	}

	pub async fn delete_text(&self, key: &str) -> Result<()> {
		let url = format!("{}/text/{key}", self.base_url).to_owned();
		let response = self.client.delete(&url).send().await?;

		let status_code = response.status();
		if status_code == StatusCode::OK {
			Ok(())
		} else {
			Err(anyhow!("Failed to delete text: {status_code}"))
		}
	}

	pub async fn delete_file(&self, key: &str) -> Result<()> {
		let url = format!("{}/file/{key}", self.base_url).to_owned();
		let response = self.client.delete(&url).send().await?;

		let status_code = response.status();
		if status_code == StatusCode::OK {
			Ok(())
		} else {
			Err(anyhow!("Failed to delete file: {status_code}"))
		}
	}

	pub async fn upsert_share(&self, share: UpsertShareRequest) -> Result<()> {
		let url = format!("{}/share/", self.base_url).to_owned();
		let response = self.client.post(&url).json(&share).send().await?;

		if response.status() == StatusCode::OK {
			Ok(())
		} else {
			Err(anyhow!("Failed to create share"))
		}
	}

	pub async fn delete_share(&self, id: &str) -> Result<()> {
		let url = format!("{}/share/{id}", self.base_url).to_owned();
		let response = self.client.delete(&url).send().await?;

		if response.status() == StatusCode::OK {
			Ok(())
		} else {
			Err(anyhow!("Failed to delete share"))
		}
	}
}
