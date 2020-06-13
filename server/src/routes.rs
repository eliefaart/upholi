use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_multipart::{Multipart, Field};
use serde::{Serialize, Deserialize};
use futures::{StreamExt, TryStreamExt};

use crate::types;
use crate::database;
use crate::files;
use crate::photos;
use crate::albums;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatedResult {
	id: String
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResult {
	message: String
}

struct FormData {
	name: String,
	bytes: Vec<u8>
}

pub async fn index() -> impl Responder {
	format!("Service running")
}

pub async fn route_get_albums() -> impl Responder {
	web::Json(database::album::get_all())
}

pub async fn route_get_album(req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();
	let result = database::album::get(album_id);

	match result {
		Some(album) => {
			let mut ids: Vec<&str> = Vec::new();

			for id in album.photos.iter() {
				ids.push(&id[..]);
			}
			
			let response = types::ClientAlbum {
				title: Some(album.title),
				thumb_photo: {
					if let Some(thumb_photo_id) = album.thumb_photo_id { 
						let result = database::photo::get(&thumb_photo_id);
						match result {
							Some(thumb_photo) => Some(thumb_photo.to_client_photo()),
							None => None
						}
					} else { 
						None 
					}
				},
				photos: {
					let result = database::photo::get_many(&ids);
					if let Some(photos) = result {
						let mut result_photos = Vec::new();
						for photo in photos {
							result_photos.push(photo.to_client_photo());
						}
	
						Some(result_photos)
					} else {
						None
					}
				}
			};
			HttpResponse::Ok().json(response)
		},
		None => create_not_found_response()
	}
}

pub async fn route_create_album(album: web::Json<albums::Album>) -> impl Responder {
	// TODO: Create different struct for request input. Should only have title, maybe other, but definetly not ID.
	let album = albums::Album::create(&album.title);
	let result = database::album::insert(&album);

	match result {
		Ok(_) => HttpResponse::Ok().json(CreatedResult{id: album.id}),
		Err(error) => create_internal_server_error_response(&error)
	}
}

pub async fn route_update_album(req: HttpRequest, album: web::Json<types::UpdateAlbum>) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	// TODO: Verify if all photoIds & thumbPhotoId are valid.

	let result = database::album::update(&album_id, &album);
	match result {
		Some(_) => create_ok_response(),
		None => create_not_found_response()
	}
}

pub async fn route_delete_album(req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();
	let result = database::album::delete(&album_id);

	match result {
		Some(_) => create_ok_response(),
		None => create_not_found_response()
	}
}

pub async fn route_get_photos() -> impl Responder {
	web::Json(database::photo::get_all())
}


/// Delete a single photo
pub async fn route_delete_photo(req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	delete_photos(&[photo_id])
}

/// Delete multiple photos
pub async fn route_delete_photos(photo_ids: web::Json<Vec<String>>) -> impl Responder {
	let mut ids: Vec<&str> = Vec::new();
	for id in photo_ids.iter() {
		ids.push(&id);
	}

	delete_photos(&ids)
}

pub async fn route_get_photo(req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(photo_id);

	match result {
		Some(photo) => HttpResponse::Ok().json(photo),
		None => create_not_found_response()
	}
}

pub async fn route_download_photo_thumbnail(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(&_photo_id);

	match result {
		Some(photo_info) => serve_photo(&photo_info.path_thumbnail, &photo_info.name),
		None => create_not_found_response()
	}
}

pub async fn route_download_photo_preview(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(&_photo_id);

	match result {
		Some(photo_info) => serve_photo(&photo_info.path_preview, &photo_info.name),
		None => create_not_found_response()
	}
}

pub async fn route_download_photo_original(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(&_photo_id);

	match result {
	Some(photo_info) => serve_photo(&photo_info.path_original, &photo_info.name),
		None => create_not_found_response()
	}
}

/// Create an HTTP response that returns a file from disk
fn serve_photo(path: &str, filename: &str) -> actix_http::Response {
	let result = files::get_photo(path);

	match result {
		Some(file_bytes) => {
			HttpResponse::Ok()
				.content_type("image/jpeg")
				.header(http::header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", filename))
				.body(file_bytes)
		},
		None => create_internal_server_error_response("Error reading file content from disk, or file not found")
	}
}

pub async fn route_upload_photo(payload: Multipart) -> impl Responder {
	let form_data = get_form_data(payload).await;

	let mut files_iter = form_data.iter().filter(|d| d.name == "file");
	let file_option = files_iter.next();
	let remaining_files = files_iter.count();

	if remaining_files > 0 {
		return create_bad_request_response("Request contains more than one file.");
	}

	match file_option {
		Some(file) => {
			let result = photos::Photo::create(&file.bytes);
			match result {
				Ok(photo) => {
					let result = database::photo::insert(&photo);
					match result {
						Ok(_) => HttpResponse::Ok().json(CreatedResult{id: photo.id}),
						Err(error) => create_bad_request_response(&error)
					}
				},
				Err(error) => create_bad_request_response(&error)
			}
		},
		None => create_bad_request_response("Request contains no file.")
	}
}

/// Gets all fields from multipart payload.
async fn get_form_data(mut payload: Multipart) -> Vec<FormData> {
	let mut form_data: Vec<FormData> = Vec::new();

	while let Ok(Some(field)) = payload.try_next().await {
		
		let content_disposition = field.content_disposition().unwrap();
		//let content_type = field.content_type();
		let key = content_disposition.get_name().unwrap();

		let field_bytes = get_form_field_bytes(field).await;
		form_data.push(FormData{
			name: key.to_string(), 
			bytes: field_bytes
		});
	}

	form_data
}

/// Gets the bytes of a single multipart field.
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

/// Delete multiple photos from database and disk
fn delete_photos(ids: &[&str]) -> impl Responder {
	// Delete physical files for photo
	for id in ids {
		delete_photo_files(&id);
	}
	
	match database::photo::delete_many(ids) {
		Ok(_) => create_ok_response(),
		Err(_) => create_not_found_response()
	}
}

/// Deletes all physical files of a photo from file system
/// Original, thumbnail and preview images.
fn delete_photo_files(photo_id: &str) {
	if let Some(photo) = database::photo::get(&photo_id) {
		files::delete_photo(&photo.path_original);
		files::delete_photo(&photo.path_preview);
		files::delete_photo(&photo.path_thumbnail);
	}
}

/// Create a HTTP 200 OK response
fn create_ok_response() -> actix_http::Response {
	HttpResponse::Ok().finish()
}

/// Create a HTTP 404 Not Found response
fn create_not_found_response() -> actix_http::Response {
	HttpResponse::NotFound().finish()
}

/// Create a HTTP 400 Bad Request response
fn create_bad_request_response(message: &str) -> actix_http::Response {
	HttpResponse::BadRequest().json(ErrorResult{message: message.to_string()})
}

/// Create a HTTP 500 Internal Server Error response
fn create_internal_server_error_response(message: &str) -> actix_http::Response {
	HttpResponse::InternalServerError().json(ErrorResult{message: message.to_string()})
}