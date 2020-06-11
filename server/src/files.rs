use std::fs::{File};
use std::io::prelude::*;
use std::path::Path;

/// Store a photo on file system. Returns the relative path of photo
pub fn store_photo(file_name: &str, file_bytes: &[u8]) -> String {
	
	let photo_relative_path = file_name.to_string();
	let photo_absolute_path = get_absolute_photo_path(&photo_relative_path);

	let create_result = File::create(photo_absolute_path);
	let mut file = create_result.unwrap();

	let write_result = file.write_all(file_bytes);
	match write_result {
		Ok(_value) => (),
		Err(error) => println!("{}", error)
	}

	photo_relative_path
}

/// Retreive a bytes of a photo from file system
pub fn get_photo(photo_relative_path: &str) -> Option<Vec<u8>> {
	let photo_absolute_path = get_absolute_photo_path(photo_relative_path);
	let result = File::open(&photo_absolute_path);

	match result {
		Ok(mut file) => {
			let mut file_bytes: Vec<u8> = Vec::new();
			let result = file.read_to_end(&mut file_bytes);

			match result {
				Ok(_) => Some(file_bytes),
				Err(_) => None
			}
		},
		Err(_) => None
	}
}

/// Deletes a file from file system
pub fn delete_photo(photo_relative_path: &str) {
	let absolute_path = get_absolute_photo_path(photo_relative_path);
	std::fs::remove_file(absolute_path).unwrap_or_default();
}

/// Returns the absolute path for given relative photo path
fn get_absolute_photo_path(photo_relative_path: &str) -> String {
	Path::new(&get_photos_base_path()).join(photo_relative_path).to_str().unwrap().to_string()
}

/// Returns the absolute path to photo storage base directory 
/// TODO: Can probably make this lazy_static.. path is constant and only need to check+create once
fn get_photos_base_path() -> String {
	
	const PHOTOS_FOLDER_NAME: &str = "photos";

	let path_info = Path::new(&crate::SETTINGS.photos.base_directory);
	if !path_info.exists() {
		panic!("Path {} does not exist", &crate::SETTINGS.photos.base_directory);
	}

	let photos_path_info = path_info.join(&PHOTOS_FOLDER_NAME);
	if !photos_path_info.exists() {
		let result = std::fs::create_dir(&photos_path_info);
		if result.is_err() {
			panic!("Failed to create directory {}", photos_path_info.to_str().unwrap().to_string());
		}
	}

	photos_path_info.to_str().unwrap().to_string()
}