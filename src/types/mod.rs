use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Album {
	pub id: u32,
	pub title: String
}

#[derive(Serialize, Deserialize)]
pub struct BsonAlbum {
	#[serde(rename = "_id")]
    pub id: bson::oid::ObjectId,
	pub title: String
}

impl Photo {
	pub fn to_bson_photo(&self) -> BsonPhoto {
		// Try parse ID as object_id, otherwise generate new object_id
		let bson_object_id: bson::oid::ObjectId;
		let object_id_result = bson::oid::ObjectId::with_string(&self.id);
		match object_id_result {
			Ok(object_id) => bson_object_id = object_id,
			Err(_) => bson_object_id = bson::oid::ObjectId::new().unwrap()
		}
	
		BsonPhoto{
			id: bson_object_id,
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

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

	#[test]
	fn test_bson_photo_to_photo_generated_id() {

		let photo = create_dummy_photo_with_id("_this_will_cause_one_to_be_generated");
		let bson_photo = photo.to_bson_photo();

		assert_eq!(bson_photo.name, photo.name);
		assert_eq!(bson_photo.width, photo.width as i32);
		assert_eq!(bson_photo.height, photo.height as i32);
		assert_eq!(bson_photo.path_thumbnail, photo.path_thumbnail);
		assert_eq!(bson_photo.path_preview, photo.path_preview);
		assert_eq!(bson_photo.path_original, photo.path_original);
	}

	#[test]
	fn test_bson_photo_to_photo_set_id() {

		let object_id = bson::oid::ObjectId::new().unwrap();
		let object_id_hex = object_id.to_hex();

		let photo = create_dummy_photo_with_id(&object_id_hex);
		let bson_photo = photo.to_bson_photo();

		assert_eq!(bson_photo.name, photo.name);
		assert_eq!(bson_photo.width, photo.width as i32);
		assert_eq!(bson_photo.height, photo.height as i32);
		assert_eq!(bson_photo.path_thumbnail, photo.path_thumbnail);
		assert_eq!(bson_photo.path_preview, photo.path_preview);
		assert_eq!(bson_photo.path_original, photo.path_original);
	}

	#[test]
	fn test_photo_to_bson_photo() {

		let object_id = bson::oid::ObjectId::new().unwrap();
		let object_id_hex = object_id.to_hex();

		let bson_photo = create_dummy_bson_photo_with_id(&object_id);
		let photo = bson_photo.to_photo();

		assert_eq!(photo.id, object_id_hex);
		assert_eq!(photo.name, bson_photo.name);
		assert_eq!(photo.width as i32, bson_photo.width);
		assert_eq!(photo.height as i32, bson_photo.height);
		assert_eq!(photo.path_thumbnail, bson_photo.path_thumbnail);
		assert_eq!(photo.path_preview, bson_photo.path_preview);
		assert_eq!(photo.path_original, bson_photo.path_original);
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
}