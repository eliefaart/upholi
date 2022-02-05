use crate::error::Result;
use azure_core::HttpClient;
use azure_storage::clients::{AsStorageClient, StorageAccountClient, StorageClient};
use azure_storage_blobs::prelude::{
	AsBlobClient, AsBlobServiceClient, AsContainerClient, BlobClient, BlobServiceClient, ContainerClient,
};
use std::sync::Arc;

pub struct AzureStorageProvider {
	storage_client: Arc<StorageClient>,
	blob_client: Arc<BlobServiceClient>,
}

impl AzureStorageProvider {
	pub fn new() -> AzureStorageProvider {
		let account_name = &crate::SETTINGS.storage.azure_storage_account_name;
		let account_key = &crate::SETTINGS.storage.azure_storage_account_key;

		let reqwest_client = reqwest::Client::new();
		let http_client: Arc<dyn HttpClient> = Arc::new(reqwest_client);
		let storage_client = StorageAccountClient::new_access_key(http_client, account_name, account_key).as_storage_client();

		AzureStorageProvider {
			storage_client: storage_client.clone(),
			blob_client: storage_client.as_blob_service_client(),
		}
	}

	pub async fn store_file(&self, container: &str, name: &str, bytes: &[u8]) -> Result<()> {
		let file_bytes: Vec<u8> = bytes.iter().map(|byte| byte.to_owned()).collect();

		let blob = self.get_blob_client(container, name);
		match blob.put_block_blob(file_bytes).execute().await {
			Ok(_) => Ok(()),
			Err(err) => Err(err),
		}
	}

	pub async fn get_file(&self, container: &str, name: &str) -> Result<Option<Vec<u8>>> {
		let blob = self.get_blob_client(container, name);
		match blob.get().execute().await {
			Ok(result) => Ok(Some(result.data.to_vec())),
			Err(err) => Err(err),
		}
	}

	pub async fn delete_file(&self, container: &str, name: &str) -> Result<()> {
		let blob = self.get_blob_client(container, name);
		match blob.delete().execute().await {
			Ok(_) => Ok(()),
			Err(err) => Err(err),
		}
	}

	fn get_blob_client(&self, container_name: &str, blob_name: &str) -> Arc<BlobClient> {
		self.get_container_client(container_name).as_blob_client(String::from(blob_name))
	}

	fn get_container_client(&self, container_name: &str) -> Arc<ContainerClient> {
		self.storage_client.as_container_client(String::from(container_name))
	}

	/// Create container with given name, if it doesn't already exist.
	pub async fn create_container_if_not_exists(&self, container_name: &str) -> Result<()> {
		match self.blob_client.list_containers().prefix(container_name).execute().await {
			Ok(containers) => {
				let container_exists = containers
					.incomplete_vector
					.iter()
					.any(|container| container.name == container_name);
				if !container_exists {
					let container_client = self.get_container_client(container_name);
					match container_client.create().execute().await {
						Ok(_) => Ok(()),
						Err(err) => Err(err),
					}
				} else {
					Ok(())
				}
			}
			Err(err) => Err(err),
		}
	}
}
