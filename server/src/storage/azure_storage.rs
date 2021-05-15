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



	pub fn store_file(&self, file_bytes: &[u8]) -> Result<String> {
		Ok("".to_string())
	}

	pub async fn get_file(&self, file_id: &str) -> Result<Option<Vec<u8>>> {




		// let http_client: Arc<Box<dyn HttpClient>> = Arc::new(Box::new(reqwest::Client::new()));
		// let storage_account =
		// 	StorageAccountClient::new_access_key(http_client.clone(), &self.storage_account_name, &self.storage_account_key)
		// 		.as_storage_client();
		// //let container = storage_account.as_container_client(container_name);

		// let containers = storage_account.list_containers().execute().await;

		//let max_results = NonZeroU32::new(3).unwrap();
		// let iv = storage_account
		// 	.list_containers()
		// 	.max_results(max_results)
		// 	.execute()
		// 	.await?;
		// println!(
		// 	"List containers returned {} containers.",
		// 	iv.incomplete_vector.len()
		//);







		Ok(Some(vec!{}))
	}

	pub fn delete_file(&self, file_id: &str) -> Result<()> {
		Ok(())
	}
}