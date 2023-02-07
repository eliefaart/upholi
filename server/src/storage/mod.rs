use crate::model::User;
use anyhow::Result;
use lazy_static::lazy_static;

mod azure_storage;
mod local_disk;

lazy_static! {
    static ref STORAGE_PROVIDER: StorageProvider = match crate::SETTINGS.storage.provider {
        crate::settings::StorageProvider::Disk => StorageProvider::Disk(local_disk::LocalDiskStorageProvider::new()),
        crate::settings::StorageProvider::Azure => StorageProvider::Azure(azure_storage::AzureStorageProvider::new()),
    };
}

enum StorageProvider {
    Disk(local_disk::LocalDiskStorageProvider),
    Azure(azure_storage::AzureStorageProvider),
}

/// Get storage provider
fn get_provider<'a>() -> &'a StorageProvider {
    &STORAGE_PROVIDER
}

/// Initialize storage for user, e.g. preparing directories.
pub async fn init_storage_for_user(user: &User) -> Result<()> {
    match get_provider() {
        StorageProvider::Azure(azure) => azure.create_container_if_not_exists(&user.id).await,
        _ => Ok(()),
    }
}

/// Store a file
/// Returns a unique id for the file
pub async fn store_file(file_id: &str, owner_user_id: &str, file_bytes: &[u8]) -> Result<()> {
    // Store bytes
    match get_provider() {
        StorageProvider::Disk(disk) => disk.store_file(file_id, file_bytes),
        StorageProvider::Azure(azure) => azure.store_file(owner_user_id, file_id, file_bytes).await,
    }
}

/// Retreive file contents
pub async fn get_file(file_id: &str, owner_user_id: &str) -> Result<Option<Vec<u8>>> {
    let bytes = match get_provider() {
        StorageProvider::Disk(disk) => disk.get_file(file_id),
        StorageProvider::Azure(azure) => azure.get_file(owner_user_id, file_id).await,
    }?;

    match bytes {
        Some(bytes) => Ok(Some(bytes)),
        None => Ok(None),
    }
}

/// Delete a file
pub async fn delete_file(file_id: &str, owner_user_id: &str) -> Result<()> {
    match get_provider() {
        StorageProvider::Disk(disk) => disk.delete_file(file_id),
        StorageProvider::Azure(azure) => azure.delete_file(owner_user_id, file_id).await,
    }
}
