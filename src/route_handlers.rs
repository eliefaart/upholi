use http::{StatusCode};
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_multipart::{Multipart, Field};
use serde::{Serialize, Deserialize};
use image::GenericImageView;
use futures::{StreamExt, TryStreamExt};

use crate::database;
use crate::images;
use crate::files;

const DIMENSIONS_THUMB: u32 = 400;
const DIMENSIONS_PREVIEW: u32 = 1500;

#[derive(Serialize)]
struct Album {
	id: u32,
	title: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	name: String,
	//content_type: String,
	base64: String
}

struct FormData {
	name: String,
	bytes: Vec<u8>
}

pub async fn handle_greet(req: HttpRequest) -> impl Responder {
	let name = req.match_info().get("name").unwrap_or("World");
	format!("Hello {}!", &name)
}

pub async fn handle_get_albums() -> impl Responder {
	let albums = get_albums();
	web::Json(albums)
}

pub async fn handle_insert_album() -> impl Responder {
	HttpResponse::build(StatusCode::OK)
}

pub async fn test_upload_photo(payload: Multipart) -> Result<HttpResponse, Error> {

	let form_data = get_form_data(payload).await;
	for data in form_data {
		println!("\t{}: {}", data.name, data.bytes.len());

		if data.name == "file" {
			let file_name = "TODO.jpg".to_string();

			let original_path = format!("C:/Development/_TestData/hummingbird/photos/{}", file_name);
			let thumbnail_path = format!("C:/Development/_TestData/hummingbird/photos/thumb_{}", file_name);
			let preview_path = format!("C:/Development/_TestData/hummingbird/photos/preview_{}", file_name);

			let thumbnail_file_name = format!("thumb_{}", file_name);
			let preview_file_name = format!("preview_{}", file_name);

			let thumbnail_image_bytes = images::resize_image(&data.bytes, DIMENSIONS_THUMB);
			let preview_image_bytes = images::resize_image(&data.bytes, DIMENSIONS_PREVIEW);
			
			files::store_photo(&file_name.to_string(), &data.bytes);
			files::store_photo(&thumbnail_file_name.to_string(), &thumbnail_image_bytes);
			files::store_photo(&preview_file_name.to_string(), &preview_image_bytes);

			let (photo_width, photo_height) = images::get_image_dimensions(&data.bytes);

			let photo = database::Photo {
				name: file_name.to_string(),
				width: photo_width,
				height: photo_height,
				path_thumbnail: thumbnail_path,
				path_preview: preview_path,
				path_original: original_path
			};
		
			database::add_photo(photo);
		}
	}

	Ok(HttpResponse::Ok().into())
}

// Gets all fields from multipart payload.
async fn get_form_data(mut payload: Multipart) -> Vec<FormData> {

	let mut form_data: Vec<FormData> = Vec::new();

	while let Ok(Some(field)) = payload.try_next().await {
		
		let content_disposition = field.content_disposition().unwrap();
		let content_type = field.content_type();
		let name = content_disposition.get_name().unwrap();

		println!("name: {}", name);
		println!("content_type: {}", content_type);
		println!("content_disposition: {:?}", content_disposition);

		let field_bytes = get_form_field_bytes(field).await;
		form_data.push(FormData{name: name.to_string(), bytes: field_bytes});
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

pub async fn handle_insert_photo(photo_json: web::Json<Photo>) -> impl Responder {

	const DIMENSIONS_THUMB: u32 = 300;
	const DIMENSIONS_PREVIEW: u32 = 1250;

	// std::env::current_dir().unwrap().to_string_lossy()
	let photo_name = &photo_json.name;

	// Convert base64 string to image
	let photo_bytes = images::from_base64(&photo_json.base64);
	let image_result = image::load_from_memory(&photo_bytes[0..]);
	let image = image_result.unwrap();

	let image_path = format!("C:/Development/_TestData/hummingbird/photos/{}", photo_name);
	let mut thumbnail_path = format!("C:/Development/_TestData/hummingbird/photos/thumb_{}", photo_name);
	let mut preview_path = format!("C:/Development/_TestData/hummingbird/photos/preview_{}", photo_name);

	// Save original image to disk
	let save_result = image.save(&image_path);
	println!("{:?}", save_result);

	let photo_width = image.width();
	let photo_height = image.height();

	// Create thumbnail image
	if photo_width > DIMENSIONS_THUMB && photo_height > DIMENSIONS_THUMB {
		let image_thumbnail = image.thumbnail(DIMENSIONS_THUMB, DIMENSIONS_THUMB);
		let save_result = image_thumbnail.save(&thumbnail_path);
		println!("{:?}", save_result);
	} 
	else {
		thumbnail_path = String::from(&image_path);
	}

	// Create preview image
	if photo_width > DIMENSIONS_THUMB && photo_height > DIMENSIONS_THUMB {
		let image_preview = image.thumbnail(DIMENSIONS_PREVIEW, DIMENSIONS_PREVIEW);
		let save_result = image_preview.save(&preview_path);
		println!("{:?}", save_result);
	}
	else {
		preview_path = String::from(&image_path);
	}

	let photo = database::Photo{
		name: photo_name.to_string(),
		width: photo_width,
		height: photo_height,
		path_thumbnail: thumbnail_path,
		path_preview: preview_path,
		path_original: image_path
	};

	database::add_photo(photo);
	web::HttpResponse::build(StatusCode::OK)
}

fn get_albums() -> Vec<Album> {
	let mut albums: Vec<Album> = Vec::new();
	let album_titles = ["Boom", "Hello world"];

	for id in 0..album_titles.len() {
		let album_title = album_titles[id];

		albums.push(Album{ id: id as u32, title: album_title.to_string() });
	}

	albums
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

	#[test]
	fn test_get_albums() {
		let albums = get_albums();
		for album in albums.iter() {
			let album_title_length = album.title.len();

			assert!(album_title_length >= 1, "Album title is empty");
		}
	}
}