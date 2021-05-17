use std::sync::Arc;
use crate::{error::Result, ids::create_unique_id};
use azure_core::prelude::*;
use azure_storage::blob::prelude::*;
use azure_storage::core::prelude::*;

static PHOTO_CONTAINER_NAME: &'static str = "dev-photos";

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

		// tokio::spawn(async move {
		// 	AzureStorageProvider::create_container_if_not_exists(&storage_account_client, PHOTO_CONTAINER_NAME).await;
		// });

		AzureStorageProvider {
			storage_client: storage_account_client
		}
	}

	pub async fn test(&self) -> Result<()> {
		let _ = AzureStorageProvider::create_container_if_not_exists(&self.storage_client, PHOTO_CONTAINER_NAME).await;

		// println!("Blobs:");
		// let container = self.storage_client.as_container_client(PHOTO_CONTAINER_NAME);
		// // match container.delete().execute().await {
		// // 	Ok(_) => println!("deleted container"),
		// // 	Err(err) => println!("{}", err),
		// // }
		// // match container.create().execute().await {
		// // 	Ok(_) => println!("created container"),
		// // 	Err(err) => println!("{}", err),
		// // }
		// if let Ok(blobs) = container.list_blobs().execute().await {
		// 	for blob in blobs.blobs.blobs {
		// 		println!("{}", blob.name);
		// 	}
		// }



		// println!("Containers:");
		// if let Ok(containers) = self.storage_client.list_containers().execute().await {
		// 	for container in containers.incomplete_vector.iter() {
		// 		println!("{}", container.name);
		// 	}
		// }

		Ok(())
	}

	pub async fn store_file(&self, file_bytes: &[u8]) -> Result<String> {
		let file_name = create_unique_id();
		let file_bytes: Vec<u8> = file_bytes
			.iter()
			.map(|byte| byte.to_owned())
			.collect();

		let container = self.storage_client.as_container_client(PHOTO_CONTAINER_NAME);
		let blob = container.as_blob_client(&file_name);
		match blob.put_block_blob(file_bytes).execute().await {
			Ok(_) => Ok(file_name),
			Err(err) => Err(err)
		}
	}

	pub async fn get_file(&self, file_id: &str) -> Result<Option<Vec<u8>>> {
		let blob = self.get_blob_client(file_id);
		match blob.get().execute().await {
			Ok(result) => Ok(Some(result.data.to_vec())),
			Err(err) => Err(err)
		}
	}

	pub async fn delete_file(&self, file_id: &str) -> Result<()> {
		let blob = self.get_blob_client(file_id);
		match blob.delete().execute().await {
			Ok(_) => Ok(()),
			Err(err) => Err(err)
		}
	}

	fn get_blob_client(&self, blob_id: &str) -> Arc<BlobClient> {
		let container = self.storage_client.as_container_client(PHOTO_CONTAINER_NAME);
		container.as_blob_client(blob_id)
	}

	/// Create container with given name, if it doesn't already exist.
	async fn create_container_if_not_exists(storage_client: &Arc<StorageClient>, container_name: &str) -> Result<()> {
		match storage_client.list_containers().prefix(container_name).execute().await {
			Ok(containers) => {
				let container_exists = containers.incomplete_vector.iter().any(|container| container.name == container_name);
				println!("{}", container_exists);
				if !container_exists {
					let container = storage_client.as_container_client(container_name);
					match container.create().execute().await {
						Ok(_) => {
							println!("Created collection {}", container_name);
							Ok(())
						},
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