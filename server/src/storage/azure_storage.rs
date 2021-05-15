use std::sync::Arc;
use crate::{error::Result};
use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;

use super::StorageProvider;

pub struct AzureStorageProvider {
	storage_account_name: String,
	storage_account_key: String,
}

impl AzureStorageProvider { // for AzureStorageProvider {
	pub fn new() -> AzureStorageProvider {
		println!("{}", crate::SETTINGS.storage.azure_storage_account_key);
		AzureStorageProvider {
			storage_account_name: crate::SETTINGS.storage.azure_storage_account_name.to_string(),
			storage_account_key: crate::SETTINGS.storage.azure_storage_account_key.to_string()
		}
	}

	pub async fn test(&self) -> Result<()> {
		let container_name = "test";

		 let reqwest_client = Box::new(reqwest::Client::new());
		 let http_client: Arc<Box<dyn HttpClient>> = Arc::new(reqwest_client);
		 let storage_account = StorageAccountClient::new_access_key(http_client.clone(), &self.storage_account_name, &self.storage_account_key).as_storage_client();

		// println!("Blobs:");
		// let container = storage_account.as_container_client(container_name);
		// if let Ok(blobs) = container.list_blobs().execute().await {
		// 	for blob in blobs.blobs.blobs {
		// 		println!("{}", blob.name);
		// 	}
		// }

		println!("Containers:");
		if let Ok(containers) = storage_account.list_containers().execute().await {
			for container in containers.incomplete_vector.iter() {
				println!("{}", container.name);
			}
		}

		Ok(())
	}

	pub fn store_file(&self, file_bytes: &[u8]) -> Result<String> {
		Ok("".to_string())
	}

	pub async fn get_file(&self, file_id: &str) -> Result<Option<Vec<u8>>> {
		Ok(Some(vec!{}))
	}

	pub fn delete_file(&self, file_id: &str) -> Result<()> {
		Ok(())
	}
}