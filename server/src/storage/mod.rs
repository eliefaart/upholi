use lazy_static::lazy_static;
use crate::error::Result;

mod local_disk;
mod azure_storage;

lazy_static! {
	pub static ref DiskStorageProvider: local_disk::LocalDiskStorageProvider = local_disk::LocalDiskStorageProvider::new();
	pub static ref AzureStorageProvider: azure_storage::AzureStorageProvider = azure_storage::AzureStorageProvider::new();
}

pub trait StorageProvider {
	/// Store a file
	/// Returns a unique id for the file
	fn store_file(&self, file_bytes: &[u8]) -> Result<String>;

	/// Retreive file contents
	fn get_file(&self, file_id: &str) -> Result<Option<Vec<u8>>>;

	/// Delete a file
	fn delete_file(&self, file_id: &str) -> Result<()>;
}

pub async fn test() -> Result<()> {
	AzureStorageProvider.test().await
}

/// Store a file
/// Returns a unique id for the file
pub fn store_file(file_bytes: &[u8]) -> Result<String> {
	match crate::SETTINGS.storage.provider {
		crate::settings::StorageProvider::Disk => {
			DiskStorageProvider.store_file(file_bytes)
		},
		crate::settings::StorageProvider::Azure => {
			AzureStorageProvider.store_file(file_bytes)
		}
	}
}

/// Retreive file contents
pub async fn get_file(file_id: &str) -> Result<Option<Vec<u8>>> {
	match crate::SETTINGS.storage.provider {
		crate::settings::StorageProvider::Disk => {
			DiskStorageProvider.get_file(file_id)
		},
		crate::settings::StorageProvider::Azure => {
			AzureStorageProvider.get_file(file_id).await
		}
	}
}

/// Delete a file
pub fn delete_file(file_id: &str) -> Result<()> {
	match crate::SETTINGS.storage.provider {
		crate::settings::StorageProvider::Disk => {
			DiskStorageProvider.delete_file(file_id)
		},
		crate::settings::StorageProvider::Azure => {
			AzureStorageProvider.delete_file(file_id)
		}
	}
}