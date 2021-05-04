use crate::error::Result;

use super::StorageProvider;

pub struct AzureStorageProvider {}

// impl StorageProvider for LocalDiskStorageProvider {
// 	fn store_file(&self, file_bytes: &[u8]) -> Result<String> {
// 	}

// 	fn get_file(&self, file_id: &str) -> Result<Option<Vec<u8>>> {
// 	}

// 	fn delete_file(&self, file_id: &str) -> Result<()> {
// 	}
// }

impl AzureStorageProvider {
	pub fn new() -> AzureStorageProvider {
		AzureStorageProvider {}
	}
}