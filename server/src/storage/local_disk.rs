use std::{fs::File, io::prelude::*};
use std::path::Path;
use crate::{error::Result, ids::create_unique_id};

pub struct LocalDiskStorageProvider {}

impl LocalDiskStorageProvider {

	pub fn new() -> LocalDiskStorageProvider {
		LocalDiskStorageProvider {}
	}

	pub fn store_file(&self, file_bytes: &[u8]) -> Result<String> {
		let file_name = Self::generate_file_name();
		let photo_absolute_path = Self::get_absolute_photo_path(&file_name)?;

		let mut file = File::create(photo_absolute_path)?;

		file.write_all(file_bytes)?;

		Ok(file_name)
	}

	pub fn get_file(&self, file_id: &str) -> Result<Option<Vec<u8>>> {
		let photo_relative_path = file_id;
		let photo_absolute_path = Self::get_absolute_photo_path(photo_relative_path)?;
		let mut file = File::open(&photo_absolute_path)?;

		let mut file_bytes: Vec<u8> = Vec::new();
		file.read_to_end(&mut file_bytes)?;
		Ok(Some(file_bytes))
	}

	pub fn delete_file(&self, file_id: &str) -> Result<()> {
		let photo_relative_path = file_id;
		let absolute_path = Self::get_absolute_photo_path(photo_relative_path)?;
		std::fs::remove_file(absolute_path)?;
		Ok(())
	}

	/// Returns the absolute path for given relative photo path
	fn get_absolute_photo_path(photo_relative_path: &str) -> Result<String> {
		let absolute_path = Self::get_photos_base_path()?;
		let path_base = Path::new(&absolute_path);
		let path_photo = path_base.join(photo_relative_path);

		Ok(path_photo.to_str()
			.ok_or("Empty directory path")?
			.to_string())
	}

	/// Returns the absolute path to photo storage base directory
	/// TODO: Can probably make this lazy_static.. path is constant and only need to check+create once
	fn get_photos_base_path() -> Result<String> {
		let path_info = Path::new(&crate::SETTINGS.storage.directory_photos);
		if !path_info.exists() {
			return Err(Box::from(format!("Path {} does not exist", &crate::SETTINGS.storage.directory_photos)));
		}

		let photos_path_str = path_info.to_str().ok_or("Empty directory path")?;
		let photos_path = photos_path_str.to_string();

		if !path_info.exists() {
			let result = std::fs::create_dir(&path_info);
			if result.is_err() {
				return Err(Box::from(format!("Failed to create directory {}", photos_path)));
			}
		}

		Ok(photos_path)
	}

	fn generate_file_name() -> String {
		create_unique_id()
	}
}