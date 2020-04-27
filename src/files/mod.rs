use std::fs::File;
use std::io::prelude::*;

pub fn store_photo(file_name: &String, file_bytes: &Vec<u8>) {
	let photos_directory = "C:/Development/_TestData/hummingbird/photos/";
	let file_path = format!("{}/{}", photos_directory, file_name);

	let create_result = File::create(file_path);
	let mut file = create_result.unwrap();

	let write_result = file.write_all(file_bytes);
	//let 
}