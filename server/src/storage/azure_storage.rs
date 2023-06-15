use anyhow::{anyhow, Result};
use azure_storage::StorageCredentials;
use azure_storage_blobs::prelude::{BlobClient, BlobServiceClient, ContainerClient};
use futures::StreamExt;

pub struct AzureStorageProvider {
    blob_client: BlobServiceClient,
}

impl AzureStorageProvider {
    pub fn new() -> AzureStorageProvider {
        let account_name = &crate::SETTINGS.storage.azure_storage_account_name;
        let account_key = &crate::SETTINGS.storage.azure_storage_account_key;

        let credentials = StorageCredentials::Key(account_name.clone(), account_key.clone());
        let blob_client = BlobServiceClient::builder(account_name.clone(), credentials).blob_service_client();

        AzureStorageProvider { blob_client }
    }

    pub async fn store_file(&self, container: &str, name: &str, bytes: &[u8]) -> Result<()> {
        let file_bytes: Vec<u8> = bytes.iter().map(|byte| byte.to_owned()).collect();

        let blob = self.get_blob_client(container, name);
        blob.put_block_blob(file_bytes)
            .await
            .map_err(|error| anyhow!("{error:?}"))?;
        Ok(())
    }

    pub async fn get_file(&self, container: &str, name: &str) -> Result<Option<Vec<u8>>> {
        let blob = self.get_blob_client(container, name);
        let bytes = blob.get_content().await?;
        Ok(Some(bytes))
    }

    pub async fn delete_file(&self, container: &str, name: &str) -> Result<()> {
        let blob = self.get_blob_client(container, name);
        blob.delete().into_future().await?;
        Ok(())
    }

    fn get_blob_client(&self, container_name: &str, blob_name: &str) -> BlobClient {
        self.get_container_client(container_name).blob_client(blob_name)
    }

    fn get_container_client(&self, container_name: &str) -> ContainerClient {
        self.blob_client.container_client(container_name)
    }

    /// Create container with given name, if it doesn't already exist.
    pub async fn create_container_if_not_exists(&self, container_name: &str) -> Result<()> {
        let container_exists = self.container_exists(container_name).await?;
        if !container_exists {
            let container_client = self.get_container_client(container_name);
            container_client.create().await.map_err(|error| anyhow!("{error:?}"))
        } else {
            Ok(())
        }
    }

    /// Create container with given name, if it doesn't already exist.
    pub async fn container_exists(&self, container_name: &str) -> Result<bool> {
        let mut stream = self
            .blob_client
            .list_containers()
            .prefix(container_name.to_string())
            .into_stream();

        while let Some(page) = stream.next().await {
            let containers = page?.containers;
            if containers.iter().any(|container| container.name == container_name) {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
