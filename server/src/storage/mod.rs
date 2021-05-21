use crate::entities::user::User;
use lazy_static::lazy_static;
use crate::error::Result;

mod local_disk;
mod azure_storage;

lazy_static! {
	pub static ref DISK_STORAGE_PROVIDER: local_disk::LocalDiskStorageProvider = local_disk::LocalDiskStorageProvider::new();
	pub static ref AZURE_STORAGE_PROVIDER: azure_storage::AzureStorageProvider = azure_storage::AzureStorageProvider::new();
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

pub async fn init_storage_for_user(user: &User) -> Result<()> {
	match crate::SETTINGS.storage.provider {
		crate::settings::StorageProvider::Disk => Ok(()),
		crate::settings::StorageProvider::Azure => {
			AZURE_STORAGE_PROVIDER.create_container(&user.id).await
		}
	}
}

/// Store a file
/// Returns a unique id for the file
pub async fn store_file(file_id: &str, owner_user_id: &str, file_bytes: &[u8]) -> Result<String> {
	match crate::SETTINGS.storage.provider {
		crate::settings::StorageProvider::Disk => {
			DISK_STORAGE_PROVIDER.store_file(file_bytes)
		},
		crate::settings::StorageProvider::Azure => {
			AZURE_STORAGE_PROVIDER.store_file(owner_user_id, file_id, file_bytes).await?;
			Ok(file_id.to_string())
		}
	}
}

/// Retreive file contents
pub async fn get_file(file_id: &str, owner_user_id: &str) -> Result<Option<Vec<u8>>> {
	match crate::SETTINGS.storage.provider {
		crate::settings::StorageProvider::Disk => {
			DISK_STORAGE_PROVIDER.get_file(file_id)
		},
		crate::settings::StorageProvider::Azure => {
			AZURE_STORAGE_PROVIDER.get_file(owner_user_id, file_id).await
		}
	}
}

/// Delete a file
pub async fn delete_file(file_id: &str, owner_user_id: &str) -> Result<()> {
	match crate::SETTINGS.storage.provider {
		crate::settings::StorageProvider::Disk => {
			DISK_STORAGE_PROVIDER.delete_file(file_id)
		},
		crate::settings::StorageProvider::Azure => {
			AZURE_STORAGE_PROVIDER.delete_file(owner_user_id, file_id).await
		}
	}
}