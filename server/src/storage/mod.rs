use crate::error::Result;

mod local_disk;
mod azure_storage;

pub trait StorageProvider {
	/// Store a file
	/// Returns a unique id for the file
	fn store_file(&self, file_bytes: &[u8]) -> Result<String>;

	/// Retreive file contents
	fn get_file(&self, file_id: &str) -> Result<Option<Vec<u8>>>;

	/// Delete a file
	fn delete_file(&self, file_id: &str) -> Result<()>;
}

// Get the implementation of the storage provider trait
pub fn get_storage_provider() -> impl StorageProvider {
	local_disk::LocalDiskStorageProvider::new()
}