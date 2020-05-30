use std::fs::File;
use std::io::prelude::*;

// Store a photo on file system. Returns the relative path of photo
pub fn store_photo(file_name: &str, file_bytes: &Vec<u8>) -> String {
	let photos_folder_name = "photos";
	let photo_relative_path = format!("{}/{}", photos_folder_name, file_name);
	let photo_absolute_path = format!("{}/{}", &crate::SETTINGS.photos.base_directory, photo_relative_path);

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
	let photo_absolute_path = format!("{}/{}", &crate::SETTINGS.photos.base_directory, photo_relative_path);
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