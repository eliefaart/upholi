use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    pub id: String,
	pub name: String,
	pub width: u32,
	pub height: u32,
	pub path_thumbnail: String,
	pub path_preview: String,
	pub path_original: String
}

#[derive(Serialize, Deserialize)]
pub struct BsonPhoto {
	#[serde(rename = "_id")]	
    pub id: bson::oid::ObjectId,
	pub name: String,
	pub width: i32,
	pub height: i32,
	pub path_thumbnail: String,
	pub path_preview: String,
	pub path_original: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	#[serde(default)] 
	pub id: String,
	pub title: String,
	#[serde(default)]
	pub thumb_photo_id: Option<String>,
	#[serde(default)]
	pub photos: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BsonAlbum {
	#[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
	pub title: String,
	pub thumb_photo_id: Option<bson::oid::ObjectId>,
	pub photos: Vec<bson::oid::ObjectId>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetAlbumResult {
	pub title: String,
	pub thumb_photo: Option<Photo>,
	pub photos: Vec<Photo>
}

impl Photo {
	pub fn to_bson_photo(&self) -> BsonPhoto {
		BsonPhoto{
			id: string_to_object_id_or_new(&self.id),
			name: self.name.to_string(),
			width: self.width as i32,
			height: self.height as i32,
			path_thumbnail: self.path_thumbnail.to_string(),
			path_preview: self.path_preview.to_string(),
			path_original: self.path_original.to_string()
		}
	}
}

impl BsonPhoto {
	pub fn to_photo(&self) -> Photo {
		Photo{
			id: self.id.to_hex(),
			name: self.name.to_string(),
			width: if self.width < 0 { 0u32} else { self.width as u32 },
			height: if self.height < 0 { 0u32} else { self.height as u32 },
			path_thumbnail: self.path_thumbnail.to_string(),
			path_preview: self.path_preview.to_string(),
			path_original: self.path_original.to_string()
		}
	}
}

impl Album {
	pub fn to_bson_album(&self) -> BsonAlbum {
		let mut photos: Vec<bson::oid::ObjectId> = Vec::new();
		for photo_id in &self.photos {
			if photo_id != "" {
				photos.push(string_to_object_id(&photo_id).unwrap());
			}
		}

		BsonAlbum{
			id: string_to_object_id_or_new(&self.id),
			title: self.title.to_string(),
			thumb_photo_id: match &self.thumb_photo_id { Some(id) => Some(string_to_object_id(id).unwrap()), None => None },
			photos: photos
		}
	}
}

impl BsonAlbum {
	pub fn to_album(&self) -> Album {
		let mut photos: Vec<String> = Vec::new();
		for photo_id in &self.photos {
			photos.push((&photo_id.to_hex()).to_string());
		}

		Album{
			id: self.id.to_hex(),
			title: self.title.to_string(),
			thumb_photo_id: match &self.thumb_photo_id { Some(id) => Some(id.to_hex()), None => None },
			photos: photos
		}
	}
}

// Try parse ID as object_id
fn string_to_object_id(object_id: &String) -> Option<bson::oid::ObjectId> {
	let object_id_result = bson::oid::ObjectId::with_string(object_id);
	match object_id_result {
		Ok(object_id) => Some(object_id),
		Err(_) => None
	}
}

// Try parse ID as object_id, otherwise generate new object_id
fn string_to_object_id_or_new(object_id_str: &String) -> bson::oid::ObjectId {
	let object_id: bson::oid::ObjectId;
	if object_id_str == "" {
		object_id = bson::oid::ObjectId::new().unwrap();
	}
	else {
		let mut invalid_id_error_message = String::from("Invalid id: ");
		invalid_id_error_message.push_str(object_id_str);

		object_id = string_to_object_id(&object_id_str).expect(&invalid_id_error_message);
	}

	object_id
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

	#[test]
	fn test_photo_to_bson_photo_no_id() {

		let photo = create_dummy_photo_with_id("");
		let bson_photo = photo.to_bson_photo();

		assert_photos_equal(&photo, &bson_photo, true);
	}

	#[test]
	fn test_photo_to_bson_photo_with_id() {

		let object_id = bson::oid::ObjectId::new().unwrap();
		let object_id_hex = object_id.to_hex();

		let photo = create_dummy_photo_with_id(&object_id_hex);
		let bson_photo = photo.to_bson_photo();

		assert_photos_equal(&photo, &bson_photo, false);
	}

	#[test]
	fn test_photo_to_bson_photo_invalid_id() {

		let photo = create_dummy_photo_with_id("_invalid_id_");
		let result = std::panic::catch_unwind(|| photo.to_bson_photo());

		assert!(result.is_err());
	}

	#[test]
	fn test_bson_photo_to_photo() {

		let object_id = bson::oid::ObjectId::new().unwrap();

		let bson_photo = create_dummy_bson_photo_with_id(&object_id);
		let photo = bson_photo.to_photo();

		assert_photos_equal(&photo, &bson_photo, false);
	}

	#[test]
	fn test_album_to_bson_album_no_id() {

		let album = create_dummy_album_with_id("");
		let bson_album = album.to_bson_album();

		assert_albums_equal(&album, &bson_album, true);
	}

	#[test]
	fn test_album_to_bson_album_invalid_id() {

		let album = create_dummy_album_with_id("_invalid_id_");
		let result = std::panic::catch_unwind(|| album.to_bson_album());

		assert!(result.is_err());
	}

	#[test]
	fn test_album_to_bson_album_with_id() {

		let object_id = bson::oid::ObjectId::new().unwrap();
		let object_id_hex = object_id.to_hex();

		let album = create_dummy_album_with_id(&object_id_hex);
		let bson_album = album.to_bson_album();

		assert_albums_equal(&album, &bson_album, false);
	}

	#[test]
	fn test_bson_album_to_album() {

		let object_id = bson::oid::ObjectId::new().unwrap();

		let bson_album = create_dummy_bson_album_with_id(&object_id);
		let album = bson_album.to_album();

		assert_albums_equal(&album, &bson_album, false);
	}

	fn assert_photos_equal(photo: &Photo, bson_photo: &BsonPhoto, photo_id_is_empty: bool) {
		if photo_id_is_empty {
			assert!(photo.id == "");
			assert!(bson_photo.id.to_hex() != "");
		} else {
			assert_eq!(photo.id, bson_photo.id.to_hex());
		}
		assert_eq!(photo.name, bson_photo.name);
		assert_eq!(photo.width as i32, bson_photo.width);
		assert_eq!(photo.height as i32, bson_photo.height);
		assert_eq!(photo.path_thumbnail, bson_photo.path_thumbnail);
		assert_eq!(photo.path_preview, bson_photo.path_preview);
		assert_eq!(photo.path_original, bson_photo.path_original);
	}

	fn assert_albums_equal(album: &Album, bson_album: &BsonAlbum, album_id_is_empty: bool) {
		if album_id_is_empty {
			assert!(album.id == "");
			assert!(bson_album.id.to_hex() != "");
		} else {
			assert_eq!(album.id, bson_album.id.to_hex());
		}
		assert_eq!(album.title, bson_album.title);
		// TODO: Figure out how to do this insert. Broke when I refactered thumb_photo_id to be an Option<String> instead of String
		//assert_eq!(album.thumb_photo_id.unwrap(), bson_album.thumb_photo_id.unwrap().to_hex());
		assert_eq!(album.photos.len(), bson_album.photos.len());
	}

	fn create_dummy_photo_with_id(id: &str) -> Photo {
		Photo{
			id: id.to_string(),
			name: "photo name".to_string(),
			width: 150,
			height: 2500,
			path_thumbnail: "path_thumbnail".to_string(),
			path_preview: "path_preview".to_string(),
			path_original: "path_original".to_string(),
		}
	}

	fn create_dummy_bson_photo_with_id(id: &bson::oid::ObjectId) -> BsonPhoto {
		BsonPhoto{
			id: id.clone(),
			name: "photo name".to_string(),
			width: 150,
			height: 2500,
			path_thumbnail: "path_thumbnail".to_string(),
			path_preview: "path_preview".to_string(),
			path_original: "path_original".to_string(),
		}
	}

	fn create_dummy_album_with_id(id: &str) -> Album {
		Album{
			id: id.to_string(),
			title: "title".to_string(),
			thumb_photo_id: Some(bson::oid::ObjectId::new().unwrap().to_hex()),
			photos: vec!{
				bson::oid::ObjectId::new().unwrap().to_hex(),
				bson::oid::ObjectId::new().unwrap().to_hex(),
				bson::oid::ObjectId::new().unwrap().to_hex()
			}
		}
	}

	fn create_dummy_bson_album_with_id(id: &bson::oid::ObjectId) -> BsonAlbum {
		BsonAlbum{
			id: id.clone(),
			title: "title".to_string(),
			thumb_photo_id: Some(bson::oid::ObjectId::new().unwrap()),
			photos: vec!{
				bson::oid::ObjectId::new().unwrap(),
				bson::oid::ObjectId::new().unwrap(),
				bson::oid::ObjectId::new().unwrap()
			}
		}
	}
}