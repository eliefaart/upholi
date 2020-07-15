use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use chrono::prelude::*;

use crate::images;
use crate::files;
use crate::database;
use crate::types;
use crate::ids;
use crate::exif;
use crate::database::{DatabaseOperations, DatabaseBatchOperations, DatabaseUserOperations};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	pub id: String,
	pub user_id: i64,
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub created_on: chrono::DateTime<Utc>,
	pub hash: String,
	pub path_thumbnail: String,
	pub path_preview: String,
	pub path_original: String,
	pub exif: exif::Exif
}

impl Photo {
	/// Create a new photo from bytes
	pub fn new(user_id: i64, photo_bytes: &[u8]) -> Result<Self, String> {
		let id = ids::create_unique_id();
		let filename = Self::generate_filename(".jpg")?;
		let hash = Self::compute_md5_hash(photo_bytes);

		// Verify if this photo doesn't already exist by checking hash in database
		let exists = database::photo::exists_for_user(user_id, &hash)?;
		
		if exists {
			Err("Photo already exists".to_string())
		} else {
			// Parse exif data
			let exif = exif::Exif::parse_from_photo_bytes(photo_bytes)?;

			// Process image
			let exif_orientation = exif.orientation.unwrap_or(1) as u8;
			let image_info = images::Image::from_buffer(photo_bytes, exif_orientation)?;

			// Store files
			let thumbnail_file_name = format!("thumb_{}", filename);
			let preview_file_name = format!("preview_{}", filename);
					
			let path_original = files::store_photo(&filename, photo_bytes);
			let path_thumbnail = files::store_photo(&thumbnail_file_name, &image_info.bytes_thumbnail);
			let path_preview = files::store_photo(&preview_file_name, &image_info.bytes_preview);
			
			// Decide 'created' date for the photo. Use 'taken on' field from exif if available, otherwise use current time
			let created_on = {
				match exif.date_taken {
					Some(date_taken) => date_taken,
					None => Utc::now()
				}
			};

			Ok(Self {
				id,
				user_id,
				name: filename,
				width: image_info.width as i32,
				height: image_info.height as i32,
				created_on,
				hash,
				path_thumbnail,
				path_preview,
				path_original,
				exif
			})
		}
	}

	/// Convert photo to a smaller struct suitable for exposing to client
	/// TODO: Remove this and implement From trait for it
	pub fn to_client_photo(&self) -> types::ClientPhoto {
		types::ClientPhoto{
			id: self.id.to_string(),
			width: self.width,
			height: self.height
		}
	}

	/// Compute MD5 hash of bytes
	fn compute_md5_hash(bytes: &[u8]) -> String {
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

			if extension.ends_with('.') {
				return Err("Last extension character cannot be a period".to_string());
			}

			// Prepend extension with a period if it is missing
			if !extension.starts_with('.') {
				extension.insert(0, '.');
			}
		}
		
		// Concat
		Ok(format!("{}{}", name, extension))
	}
}

impl DatabaseOperations for Photo {
	fn get(id: &str) -> Option<Self> {
		let collection = database::get_collection_photos();
		database::find_one(&id, &collection)
	}

	fn insert(&self) -> Result<(), String> {
		if self.id.is_empty() {
			return Err("Photo ID not set".to_string());
		}

		let collection = database::get_collection_photos();
		
		match Self::get(&self.id) {
			Some(_) => Err(format!("A photo with id {} already exists", &self.id)),
			None => {
				let _ = database::insert_item(&collection, &self)?;
				Ok(())
			}
		}
	}

	fn update(&self) -> Result<(), String> {
		let collection = database::get_collection_photos();
		database::replace_one(&self.id, self, &collection)
	}

	fn delete(&self) -> Result<(), String> {
		let collection = database::get_collection_photos();
		match database::delete_one(&self.id, &collection) {
			Some(_) => Ok(()),
			None => Err("Failed to delete album".to_string())
		}
	}
}

impl DatabaseBatchOperations for Photo {
	fn get_with_ids(ids: &[&str]) -> Result<Vec<Self>, String> {
		let collection = database::get_collection_photos();
		database::find_many(&collection, None, Some(ids), None)
	}
}

impl DatabaseUserOperations for Photo {
	fn get_as_user(id: &str, user_id: i64) -> Result<Option<Self>, String>{
		match Self::get(id) {
			Some(photo) => {
				if photo.user_id != user_id {
					Err(format!("User {} does not have access to photo {}", user_id, photo.id))
				} else {
					Ok(Some(photo))
				}
			},
			None => Ok(None)
		}
	}

	fn get_all_as_user(user_id: i64) -> Result<Vec<Self>, String> {
		let collection = database::get_collection_photos();
		let sort = database::SortField{
			field: "createdOn", 
			ascending: false
		};
		database::find_many(&collection, Some(user_id), None, Some(&sort))
	}

	fn get_all_with_ids_as_user(ids: &[&str], user_id: i64) -> Result<Vec<Self>, String> {
		let collection = database::get_collection_photos();
		let sort = database::SortField{
			field: "createdOn", 
			ascending: false
		};
		database::find_many(&collection, Some(user_id), Some(ids), Some(&sort))
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
		assert!(!name.contains('.'), name);
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
		assert!(name.contains('.'));
		assert!(name.len() > extension.len());
	}

	fn test_generate_filename_bad_extension(extension: &str) {
		let result = Photo::generate_filename(extension);
		assert!(result.is_err());
	}

	// #[test]
	// fn new() {
	// 	const TITLE: &str = "Hello world";
	// 	const USER_ID: i64 = 100i64;

	// 	let photo = Photo::new(USER_ID, TITLE);

	// 	assert!(!photo.id.is_empty());
	// 	assert_eq!(photo.title, TITLE);
	// 	assert_eq!(photo.user_id, USER_ID);
	// }

	#[test]
	fn insert_empty_id() {
		let album = create_dummy_photo_with_id("");
		let result = album.insert();

		assert!(result.is_err());
	}

	fn create_dummy_photo_with_id(id: &str) -> Photo {
		Photo {
			id: id.to_string(),
			user_id: 0i64,
			name: "photo name".to_string(),
			width: 150,
			height: 2500,
			created_on: Utc::now(),
			hash: "abc123".to_string(),
			path_thumbnail: "path_thumbnail".to_string(),
			path_preview: "path_preview".to_string(),
			path_original: "path_original".to_string(),
			exif: crate::exif::Exif {
				manufactorer: None,
				model: None,
				aperture: None,
				exposure_time: None,
				iso: None,
				focal_length: None,
				focal_length_35mm_equiv: None,
				orientation: None,
				date_taken: None,
				gps_latitude: None,
				gps_longitude: None
			}
		}
	}
}