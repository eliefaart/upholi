use crate::entities::Session;
use serde::{Serialize, Deserialize};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use chrono::prelude::*;

use crate::error::*;
use crate::images;
use crate::files;
use crate::ids;
use crate::entities::exif;
use crate::database;
use crate::database::{Database, DatabaseEntity, DatabaseEntityBatch, DatabaseUserEntity, DatabaseExt};
use crate::entities::AccessControl;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	pub id: String,
	pub user_id: String,
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
	pub fn new(user_id: String, photo_bytes: &[u8]) -> Result<Self> {
		let id = ids::create_unique_id();
		let filename = Self::generate_filename(".jpg")?;
		let hash = Self::compute_md5_hash(photo_bytes);

		// Verify if this photo doesn't already exist by checking hash in database
		let exists = database::get_database().photo_exists_for_user(&user_id, &hash)?;

		if exists {
			Err(Box::from(EntityError::AlreadyExists))
		} else {
			// Parse exif data
			let exif = exif::Exif::parse_from_photo_bytes(photo_bytes)?;

			// Process image
			let exif_orientation = exif.orientation.unwrap_or(1) as u8;
			let image_info = images::Image::from_buffer(photo_bytes, exif_orientation)?;

			// Store files
			let thumbnail_file_name = format!("thumb_{}", filename);
			let preview_file_name = format!("preview_{}", filename);

			let path_original = files::store_photo(&filename, photo_bytes)?;
			let path_thumbnail = files::store_photo(&thumbnail_file_name, &image_info.bytes_thumbnail)?;
			let path_preview = files::store_photo(&preview_file_name, &image_info.bytes_preview)?;

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

	/// Compute MD5 hash of bytes
	fn compute_md5_hash(bytes: &[u8]) -> String {
		let mut md5_context = md5::Context::new();
		md5_context.consume(&bytes);
		let digest = md5_context.compute();
		format!("{:?}", digest)
	}

	/// Generate a random filename
	fn generate_filename(extension: &str) -> Result<String> {
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
				return Err(Box::from(FileError::InvalidFileName));
			}

			if extension.ends_with('.') {
				return Err(Box::from(FileError::InvalidFileName));
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

impl DatabaseEntity for Photo {
	fn get(id: &str) -> Result<Option<Self>> {
		database::get_database().find_one(database::COLLECTION_PHOTOS, id)
	}

	fn insert(&self) -> Result<()> {
		if self.id.is_empty() {
			return Err(Box::from(EntityError::IdMissing));
		}

		match Self::get(&self.id)? {
			Some(_) => Err(Box::from(EntityError::AlreadyExists)),
			None => {
				database::get_database().insert_one(database::COLLECTION_PHOTOS, &self)?;
				Ok(())
			}
		}
	}

	fn update(&self) -> Result<()> {
		database::get_database().replace_one(database::COLLECTION_PHOTOS, &self.id, self)
	}

	fn delete(&self) -> Result<()> {
		database::get_database().delete_one(database::COLLECTION_PHOTOS, &self.id)
	}
}

impl DatabaseEntityBatch for Photo {
	fn get_with_ids(ids: &[&str]) -> Result<Vec<Self>> {
		database::get_database().find_many(database::COLLECTION_PHOTOS, None, Some(ids), None)
	}
}

impl DatabaseUserEntity for Photo {
	fn get_as_user(id: &str, user_id: String) -> Result<Option<Self>>{
		match Self::get(id)? {
			Some(photo) => {
				if photo.user_id != user_id {
					Err(Box::from(EntityError::NoAccess))
				} else {
					Ok(Some(photo))
				}
			},
			None => Ok(None)
		}
	}

	fn get_all_as_user(user_id: String) -> Result<Vec<Self>> {
		let sort = database::SortField{
			field: "createdOn",
			ascending: false
		};
		database::get_database().find_many(database::COLLECTION_PHOTOS, Some(&user_id), None, Some(&sort))
	}

	fn get_all_with_ids_as_user(ids: &[&str], user_id: String) -> Result<Vec<Self>> {
		let sort = database::SortField{
			field: "createdOn",
			ascending: false
		};
		database::get_database().find_many(database::COLLECTION_PHOTOS, Some(&user_id), Some(ids), Some(&sort))
	}
}

impl AccessControl for Photo {
	fn can_view(&self, session: &Option<Session>) -> bool {
		if session_owns_photo(self, session) {
			return true;
		}

		// Check if photo is part of any collection,
		// if so, photo is publically accessible too.
		if let Ok(albums) = database::get_database().get_albums_with_photo(&self.id) {
			for album in albums {
				if album.can_view(session) {
					return true;
				}
			}
		}

		false
	}
    fn can_update(&self, session: &Option<Session>) -> bool {
		session_owns_photo(self, session)
	}
}

/// Check if Photo is owned by user of given session
fn session_owns_photo(photo: &Photo, session_opt: &Option<Session>) -> bool {
	if let Some(session) = session_opt {
		if let Some(user_id) = &session.user_id {
			if &photo.user_id == user_id {
				return true;
			}
		}
	}

	false
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::ids::create_unique_id;

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
	// 	const user_id: String = 100i64;

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

	#[test]
	fn can_view() {
		let session_owner = create_dummy_session(true);
		// let session_not_owner = create_dummy_session(true);
		// let session_anonymous = create_dummy_session(false);

		let mut photo = create_dummy_photo_with_id("");
		photo.user_id = session_owner.user_id.to_owned().unwrap();

		// Only the user that owns the photo may access it
		assert_eq!(photo.can_view(&Some(session_owner)), true);
		// Can't test the not-allowed situations without database..
		// assert_eq!(photo.can_view(&Some(session_not_owner)), false);
		// assert_eq!(photo.can_view(&Some(session_anonymous)), false);
		// assert_eq!(photo.can_view(&None), false);
	}

	#[test]
	fn can_update() {
		let session_owner = create_dummy_session(true);
		let session_not_owner = create_dummy_session(true);
		let session_anonymous = create_dummy_session(false);

		let mut photo = create_dummy_photo_with_id("");
		photo.user_id = session_owner.user_id.to_owned().unwrap();

		// Only the user that owns the photo may access it
		assert_eq!(photo.can_update(&Some(session_owner)), true);
		assert_eq!(photo.can_update(&Some(session_not_owner)), false);
		assert_eq!(photo.can_update(&Some(session_anonymous)), false);
		assert_eq!(photo.can_update(&None), false);
	}

	fn create_dummy_photo_with_id(id: &str) -> Photo {
		Photo {
			id: id.to_string(),
			user_id: create_unique_id(),
			name: "photo name".to_string(),
			width: 150,
			height: 2500,
			created_on: Utc::now(),
			hash: "abc123".to_string(),
			path_thumbnail: "path_thumbnail".to_string(),
			path_preview: "path_preview".to_string(),
			path_original: "path_original".to_string(),
			exif: crate::entities::exif::Exif {
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

	fn create_dummy_session(with_user: bool) -> Session {
		let mut session = Session::new();

		if with_user {
			session.user_id = Some(create_unique_id());
		}

		session
	}
}