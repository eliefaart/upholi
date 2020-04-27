use std::fs::File;
use std::io::prelude::*;

// Store a photo on file system. Returns the relative path of photo
pub fn store_photo(file_name: &String, file_bytes: &Vec<u8>) -> String {
	let app_base_directory = "C:/Development/_TestData/hummingbird";
	let photos_folder_name = "photos";
	let photo_relative_path = format!("{}/{}", photos_folder_name, file_name);
	let photo_absolute_path = format!("{}/{}", app_base_directory, photo_relative_path);

	let create_result = File::create(photo_absolute_path);
	let mut file = create_result.unwrap();

	let write_result = file.write_all(file_bytes);
	match write_result {
		Ok(_value) => (),
		Err(error) => println!("{}", error)
	}

	photo_relative_path
}