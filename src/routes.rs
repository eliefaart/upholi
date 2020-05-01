use http::{StatusCode};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_multipart::{Multipart, Field};
use serde::{Serialize, Deserialize};
use futures::{StreamExt, TryStreamExt};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

use crate::types;
use crate::database;
use crate::images;
use crate::files;

const DIMENSIONS_THUMB: u32 = 400;
const DIMENSIONS_PREVIEW: u32 = 1500;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UploadPhotoResult {
	photo_id: String
}

struct FormData {
	name: String,
	//filename: String,
	bytes: Vec<u8>
}

pub async fn route_index() -> impl Responder {
	format!("Hello world!")
}

pub async fn route_get_albums() -> impl Responder {
	HttpResponse::build(StatusCode::OK)
}

pub async fn route_insert_album() -> impl Responder {
	HttpResponse::build(StatusCode::OK)
}

pub async fn route_update_album(req: HttpRequest) -> impl Responder {
	let _album_id = req.match_info().get("album_id").unwrap();
	HttpResponse::build(StatusCode::OK)
}

pub async fn route_delete_album() -> impl Responder {
	HttpResponse::build(StatusCode::OK)
}

pub async fn route_get_photos() -> impl Responder {
	let photos = database::get_photos();
	web::Json(photos)
}

pub async fn route_get_photo(req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();
	let get_photo_result = database::get_photo(photo_id);

	match get_photo_result {
		Some(queried_photo) => web::Json(queried_photo),
		None => panic!("no photo") // How to return HTTP 404?
	}
}

pub async fn route_download_photo(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	HttpResponse::build(StatusCode::OK)
}

pub async fn route_upload_photo(payload: Multipart) -> impl Responder {
	let form_data = get_form_data(payload).await;
	let mut photo_id = String::new();

	for data in form_data {
		if data.name == "file" {
			// TODO: Generate a new filename if it already exists in database and/or disk.
			// OR, always generate one?
			// let filename = data.filename;

			// let thumbnail_file_name = format!("thumb_{}", filename);
			// let preview_file_name = format!("preview_{}", filename);

			// let thumbnail_image_bytes = images::resize_image(&data.bytes, DIMENSIONS_THUMB);
			// let preview_image_bytes = images::resize_image(&data.bytes, DIMENSIONS_PREVIEW);
			
			// let original_path = files::store_photo(&filename.to_string(), &data.bytes);
			// let thumbnail_path = files::store_photo(&thumbnail_file_name.to_string(), &thumbnail_image_bytes);
			// let preview_path = files::store_photo(&preview_file_name.to_string(), &preview_image_bytes);

			// let (photo_width, photo_height) = images::get_image_dimensions(&data.bytes);

			// let photo = types::Photo {
			// 	id: "".to_string(),
			// 	name: filename.to_string(),
			// 	width: photo_width,
			// 	height: photo_height,
			// 	path_thumbnail: thumbnail_path,
			// 	path_preview: preview_path,
			// 	path_original: original_path
			// };
		
			let photo = create_photo(&data.bytes);
			photo_id = database::add_photo(photo).unwrap();
		}
	}

	//HttpResponse::build(StatusCode::NOT_FOUND)
	web::Json(UploadPhotoResult{photo_id: photo_id})
}

pub async fn route_update_photo() -> impl Responder {
	HttpResponse::build(StatusCode::OK)
}

pub async fn route_delete_photo() -> impl Responder {
	HttpResponse::build(StatusCode::OK)
}

// Gets all fields from multipart payload.
async fn get_form_data(mut payload: Multipart) -> Vec<FormData> {
	let mut form_data: Vec<FormData> = Vec::new();

	while let Ok(Some(field)) = payload.try_next().await {
		
		let content_disposition = field.content_disposition().unwrap();
		//let content_type = field.content_type();
		let key = content_disposition.get_name().unwrap();
		//let filename = content_disposition.get_filename().unwrap_or_default();

		let field_bytes = get_form_field_bytes(field).await;
		form_data.push(FormData{
			name: key.to_string(), 
			//filename: filename.to_string(),
			bytes: field_bytes
		});
	}

	form_data
}

// Gets the bytes of a single multipart field.
async fn get_form_field_bytes(mut field: Field) -> Vec<u8> {
	let mut field_bytes: Vec<u8> = Vec::new();
				
	while let Some(chunk) = field.next().await {
		let chunk_bytes = chunk.unwrap();

		for byte in chunk_bytes {
			field_bytes.push(byte);
		}
	}

	field_bytes
}


fn create_photo(photo_bytes: &Vec<u8>) -> types::Photo {
	// Generate a random filename
	let mut filename: String = thread_rng()
		.sample_iter(&Alphanumeric)
		.take(20)
		.collect();
	filename.push_str(&".jpg");

	let thumbnail_file_name = format!("thumb_{}", filename);
	let preview_file_name = format!("preview_{}", filename);

	let thumbnail_image_bytes = images::resize_image(photo_bytes, DIMENSIONS_THUMB);
	let preview_image_bytes = images::resize_image(photo_bytes, DIMENSIONS_PREVIEW);
	
	let original_path = files::store_photo(&filename.to_string(), photo_bytes);
	let thumbnail_path = files::store_photo(&thumbnail_file_name.to_string(), &thumbnail_image_bytes);
	let preview_path = files::store_photo(&preview_file_name.to_string(), &preview_image_bytes);

	let (photo_width, photo_height) = images::get_image_dimensions(photo_bytes);

	types::Photo {
		id: String::new(),
		name: filename,
		width: photo_width,
		height: photo_height,
		path_thumbnail: thumbnail_path,
		path_preview: preview_path,
		path_original: original_path
	}
}