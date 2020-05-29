use http::{StatusCode};
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

struct FormData {
	name: String,
	bytes: Vec<u8>
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
			web::Json(response)
		},
		None => panic!("no album") // How to return HTTP 404?
	}
}

pub async fn route_create_album(album: web::Json<albums::Album>) -> impl Responder {
	// TODO: Create different struct for request input. Should only have title, maybe other, but definetly not ID.
	let album = albums::Album::create(&album.title);
	let result = database::album::insert(&album);

	match result {
		Ok(_) => web::Json(CreatedResult{id: album.id}),
		Err(error) => panic!(error)
	}
}

pub async fn route_update_album(req: HttpRequest, album: web::Json<types::UpdateAlbum>) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	// TODO: Verify if all photoIds & thumbPhotoId are valid.

	let result = database::album::update(&album_id, &album);
	match result {
		Some(_) => HttpResponse::build(StatusCode::OK),
		None => HttpResponse::build(StatusCode::NOT_FOUND)
	}
}

pub async fn route_delete_album(req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();
	let result = database::album::delete(&album_id);

	match result {
		Some(_) => HttpResponse::build(StatusCode::OK),
		None => HttpResponse::build(StatusCode::NOT_FOUND)
	}
}

pub async fn route_get_photos() -> impl Responder {
	web::Json(database::photo::get_all())
}

pub async fn route_delete_photos(photo_ids: web::Json<Vec<String>>) -> impl Responder {
	let mut ids: Vec<&str> = Vec::new();

	for id in photo_ids.iter() {
		ids.push(&id[..]);
	}

	let result = database::photo::delete_many(&ids);

	match result {
		Some(_) => HttpResponse::build(StatusCode::OK),
		None => HttpResponse::build(StatusCode::NOT_FOUND)
	}
}

pub async fn route_get_photo(req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(photo_id);

	match result {
		Some(photo) => web::Json(photo),
		None => panic!("no photo") // How to return HTTP 404, if 'good' path returns json?
	}
}

pub async fn route_download_photo_thumbnail(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(&_photo_id);

	match result {
		Some(photo_info) => serve_photo(&photo_info.path_thumbnail),
		None => panic!("File not found")
	}
}

pub async fn route_download_photo_preview(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(&_photo_id);

	match result {
		Some(photo_info) => serve_photo(&photo_info.path_preview),
		None => panic!("File not found")
	}
}

pub async fn route_download_photo_original(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(&_photo_id);

	match result {
		Some(photo_info) => serve_photo(&photo_info.path_original),
		None => panic!("File not found")
	}
}

fn serve_photo(path: &str) -> impl Responder {
	let result = files::get_photo(path);

	match result {
		Some(file_bytes) => {
			HttpResponse::Ok()
				.content_type("image/jpeg")
				.body(file_bytes)
		},
		None => panic!("Error reading file content from disk, or file not found")
	}
}

pub async fn route_upload_photo(payload: Multipart) -> impl Responder {
	let form_data = get_form_data(payload).await;

	let mut files_iter = form_data.iter().filter(|d| d.name == "file");
	let file_option = files_iter.next();
	let remaining_files = files_iter.count();

	if remaining_files > 0 {
		// HttpResponse::BadRequest() //(StatusCode::BAD_REQUEST);
		panic!("Request contains more than one file.")
	}

	match file_option {
		Some(file) => {

			let result = photos::Photo::create(&file.bytes);
			match result {
				Ok(photo) => {
					println!("{:?}", photo);
					let result = database::photo::insert(&photo);
					match result {
						Ok(_) => HttpResponse::Ok().json(CreatedResult{id: photo.id}),
						Err(error) => panic!(error)
					}
					
				},
				Err(err) => panic!(err)
			}
		},
		None => panic!("Request contains no file.")
	}
}

pub async fn route_delete_photo(req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::delete(&photo_id);
	
	match result {
		Some(_) => HttpResponse::build(StatusCode::OK),
		None => HttpResponse::build(StatusCode::NOT_FOUND)
	}
}

// Gets all fields from multipart payload.
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