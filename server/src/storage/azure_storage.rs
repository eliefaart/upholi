use std::sync::Arc;
use crate::{error::Result};
use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;

pub struct AzureStorageProvider {
	storage_client: Arc<StorageClient>
}

impl AzureStorageProvider {

	pub fn new() -> AzureStorageProvider {
		let account_name = &crate::SETTINGS.storage.azure_storage_account_name;
		let account_key = &crate::SETTINGS.storage.azure_storage_account_key;

		let reqwest_client = Box::new(reqwest::Client::new());
		let http_client: Arc<Box<dyn HttpClient>> = Arc::new(reqwest_client);
		let storage_account_client = StorageAccountClient::new_access_key(http_client.clone(), account_name, account_key).as_storage_client();

		AzureStorageProvider {
			storage_client: storage_account_client
		}
	}

	pub async fn create_container(&self, container: &str) -> Result<()> {
		AzureStorageProvider::create_container_if_not_exists(&self.storage_client, container).await
	}

	pub async fn store_file(&self, container: &str, name: &str, bytes: &[u8]) -> Result<()> {
		let file_bytes: Vec<u8> = bytes.iter()
			.map(|byte| byte.to_owned())
			.collect();

		let blob = self.get_blob_client(container, name);
		match blob.put_block_blob(file_bytes).execute().await {
			Ok(_) => Ok(()),
			Err(err) => Err(err)
		}
	}

	pub async fn get_file(&self, container: &str, name: &str) -> Result<Option<Vec<u8>>> {
		let blob = self.get_blob_client(container, name);
		match blob.get().execute().await {
			Ok(result) => Ok(Some(result.data.to_vec())),
			Err(err) => Err(err)
		}
	}

	pub async fn delete_file(&self, container: &str, name: &str) -> Result<()> {
		let blob = self.get_blob_client(container, name);
		match blob.delete().execute().await {
			Ok(_) => Ok(()),
			Err(err) => Err(err)
		}
	}

	fn get_blob_client(&self, container: &str, blob_name: &str) -> Arc<BlobClient> {
		let container = self.storage_client.as_container_client(container);
		container.as_blob_client(blob_name)
	}

	/// Create container with given name, if it doesn't already exist.
	async fn create_container_if_not_exists(storage_client: &Arc<StorageClient>, container_name: &str) -> Result<()> {
		match storage_client.list_containers().prefix(container_name).execute().await {
			Ok(containers) => {
				let container_exists = containers.incomplete_vector.iter().any(|container| container.name == container_name);
				if !container_exists {
					let container = storage_client.as_container_client(container_name);
					match container.create().execute().await {
						Ok(_) => Ok(()),
						Err(err) => Err(err)
					}
				}
				else {
					Ok(())
				}
			},
			Err(err) => Err(err)
		}
	}
}