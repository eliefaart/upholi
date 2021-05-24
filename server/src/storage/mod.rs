use crate::encryption::{encrypt, decrypt};
use crate::entities::user::User;
use lazy_static::lazy_static;
use crate::error::Result;

mod local_disk;
mod azure_storage;

lazy_static! {
	static ref STORAGE_PROVIDER: StorageProvider = match crate::SETTINGS.storage.provider {
		crate::settings::StorageProvider::Disk => StorageProvider::Disk(local_disk::LocalDiskStorageProvider::new()),crate::settings::StorageProvider::Azure => StorageProvider::Azure(azure_storage::AzureStorageProvider::new())
	};
	static ref ENCRYPTION_KEY: &'static [u8] = crate::SETTINGS.storage.encryption_key.as_bytes();
}

enum StorageProvider {
	Disk(local_disk::LocalDiskStorageProvider),
	Azure(azure_storage::AzureStorageProvider)
}

/// Get storage provider
fn get_provider<'a>() -> &'a StorageProvider {
	&*STORAGE_PROVIDER
}

/// Initialize storage for user, e.g. preparing directories.
pub async fn init_storage_for_user(user: &User) -> Result<()> {
	match get_provider() {
		StorageProvider::Azure(azure) => azure.create_container(&user.id).await,
		_ => Ok(())
	}
}

/// Store a file
/// Returns a unique id for the file
pub async fn store_file(file_id: &str, owner_user_id: &str, file_bytes: &[u8]) -> Result<String> {
	// Encrypt file bytes
	let file_bytes = &encrypt(&ENCRYPTION_KEY, &file_id.as_bytes()[0..12], file_bytes)?;

	// Store bytes
	match get_provider() {
		StorageProvider::Disk(disk) => {
			disk.store_file(file_bytes)
		},
		StorageProvider::Azure(azure) => {
			azure.store_file(owner_user_id, file_id, file_bytes).await?;
			Ok(file_id.to_string())
		}
	}
}

/// Retreive file contents
pub async fn get_file(file_id: &str, owner_user_id: &str) -> Result<Option<Vec<u8>>> {
	let bytes = match get_provider() {
		StorageProvider::Disk(disk) => disk.get_file(file_id),
		StorageProvider::Azure(azure) => azure.get_file(owner_user_id, file_id).await
	}?;

	match bytes {
		Some(bytes) => Ok(Some(decrypt(&ENCRYPTION_KEY, &file_id.as_bytes()[0..12], &bytes)?)),
		None => Ok(None)
	}
}

/// Delete a file
pub async fn delete_file(file_id: &str, owner_user_id: &str) -> Result<()> {
	match get_provider() {
		StorageProvider::Disk(disk) => disk.delete_file(file_id),
		StorageProvider::Azure(azure) => azure.delete_file(owner_user_id, file_id).await
	}
}