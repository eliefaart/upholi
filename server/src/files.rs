use std::fs::{File};
use std::io::prelude::*;
use std::path::Path;
use crate::error::{Result};

/// Store a photo on file system. Returns the relative path of photo
pub fn store_photo(file_name: &str, file_bytes: &[u8]) -> Result<String> {
	
	let photo_relative_path = file_name.to_string();
	let photo_absolute_path = get_absolute_photo_path(&photo_relative_path)?;

	let mut file = File::create(photo_absolute_path)?;

	file.write_all(file_bytes)?;

	Ok(photo_relative_path)
}

/// Retreive a bytes of a photo from file system
pub fn get_photo(photo_relative_path: &str) -> Result<Option<Vec<u8>>> {
	let photo_absolute_path = get_absolute_photo_path(photo_relative_path)?;
	let mut file = File::open(&photo_absolute_path)?;

	let mut file_bytes: Vec<u8> = Vec::new();
	file.read_to_end(&mut file_bytes)?;
	Ok(Some(file_bytes))
}

/// Deletes a file from file system
pub fn delete_photo(photo_relative_path: &str) -> Result<()> {
	let absolute_path = get_absolute_photo_path(photo_relative_path)?;
	std::fs::remove_file(absolute_path)?;
	Ok(())
}

/// Returns the absolute path for given relative photo path
fn get_absolute_photo_path(photo_relative_path: &str) -> Result<String> {
	let absolute_path = get_photos_base_path()?;
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