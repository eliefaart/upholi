use md5;
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::images;
use crate::files;
use crate::database;
use crate::types;
use crate::ids;

const DIMENSIONS_THUMB: u32 = 400;
const DIMENSIONS_PREVIEW: u32 = 1500;

/// A photo
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub id: String,
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub hash: String,
	pub path_thumbnail: String,
	pub path_preview: String,
	pub path_original: String
}

impl Photo {

	/// Create a new photo from bytes
	pub fn create(photo_bytes: &Vec<u8>) -> Result<Self, String> {
		let id = ids::create_unique_id();
		let filename = Self::generate_filename(".jpg")?;
		let hash = Self::compute_md5_hash(photo_bytes);

		// Verify if this photo doesn't already exist by checking hash in database
		let exists = database::photo::hash_exists(&hash)?;
		
		if exists {
			Err("Photo already exists".to_string())
		} else {
			let thumbnail_file_name = format!("thumb_{}", filename);
			let preview_file_name = format!("preview_{}", filename);
		
			let thumbnail_image_bytes = images::resize_image(photo_bytes, DIMENSIONS_THUMB);
			let preview_image_bytes = images::resize_image(photo_bytes, DIMENSIONS_PREVIEW);
			
			let path_original = files::store_photo(&filename.to_string(), photo_bytes);
			let path_thumbnail = files::store_photo(&thumbnail_file_name.to_string(), &thumbnail_image_bytes);
			let path_preview = files::store_photo(&preview_file_name.to_string(), &preview_image_bytes);
		
			let (width, height) = images::get_image_dimensions(photo_bytes);
		
			Ok(Self {
				id,
				name: filename,
				width: width as i32,
				height: height as i32,
				hash,
				path_thumbnail,
				path_preview,
				path_original
			})
		}
	}

	/// Convert photo to a smaller struct suitable for exposing to client
	pub fn to_client_photo(&self) -> types::ClientPhoto {
		types::ClientPhoto{
			id: self.id.to_string(),
			width: self.width,
			height: self.height
		}
	}

	/// Compute MD5 hash of bytes
	fn compute_md5_hash(bytes: &Vec<u8>) -> String {
		let mut md5_context = md5::Context::new();
		md5_context.consume(&bytes);
		let digest = md5_context.compute();
		format!("{:?}", digest)
	}

	/// Generate a random filename
	fn generate_filename(extension: &str) -> Result<String, String> {
		const NAME_LENGTH: usize = 20;

		// Generate random string
		let name: String = thread_rng()
			.sample_iter(&Alphanumeric)
			.take(NAME_LENGTH).collect();

		// Check and fix extension.
		// An empty extension is allowed.
		// If not empty, then make sure it starts with a period and only contains valid characters.
		let mut extension = extension.to_string();
		if !extension.is_empty() {
			let mut chars = extension.chars();
			let all_chars_valid = chars.all(|ch| ch.is_alphanumeric() || ch == '.');

			if !all_chars_valid {
				return Err(format!("Extension '{}' contains invalid characters", extension));
			}

			if extension.ends_with(".") {
				return Err("Last extension character cannot be a period".to_string());
			}

			// Prepend extension with a period if it is missing
			if !extension.starts_with(".") {
				extension.insert(0, '.');
			}
		}
		
		// Concat
		Ok(format!("{}{}", name, extension))
	}
}


#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn generate_filename() {
		test_generate_filename_with_extension("jpg");
		test_generate_filename_with_extension(".jpg");
		test_generate_filename_with_extension("123");
		test_generate_filename_with_extension("啊");
	}

	#[test]
	fn generate_filename_no_extension() {
		let name = Photo::generate_filename("").unwrap_or_default();

		assert!(name.len() > 0, name);
		assert!(!name.contains("."), name);
	}

	#[test]
	fn generate_filename_bad_extension() {
		test_generate_filename_bad_extension("&@");
		test_generate_filename_bad_extension(".(");
	}

	fn test_generate_filename_with_extension(extension: &str) {
		let name = Photo::generate_filename(extension).unwrap_or_default();

		assert!(name.ends_with(extension));
		assert!(!name.starts_with(extension));
		assert!(name.contains("."));
		assert!(name.len() > extension.len());
	}

	fn test_generate_filename_bad_extension(extension: &str) {
		let result = Photo::generate_filename(extension);
		assert!(result.is_err());
	}
}