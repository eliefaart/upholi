use crate::{entities::session::Session, storage::StorageProvider};
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_multipart::Multipart;

use crate::error::*;

use crate::database;
use crate::database::{Database, DatabaseExt, DatabaseEntity, DatabaseUserEntity};
use crate::web::http::*;
use crate::storage;
use crate::entities::AccessControl;
use crate::entities::user::User;
use crate::entities::photo::Photo;
use crate::web::handlers::responses::*;


/// Get all photos
pub async fn route_get_photos(user: User) -> impl Responder {
	let _ = crate::storage::test().await;
	match Photo::get_all_as_user(user.id) {
		Ok(photos) => {
			let photos_small: Vec<PhotoSmall> = photos.into_iter()
				.map(PhotoSmall::from)
				.collect();
			HttpResponse::Ok().json(photos_small)
		},
		Err(error) => {
			println!("{}", error);
			create_internal_server_error_response(Some(error))
		}
	}
}

/// Delete a single photo
pub async fn route_delete_photo(user: User, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	delete_photos(user.id, &[photo_id])
}

/// Delete multiple photos
pub async fn route_delete_photos(user: User, photo_ids: web::Json<Vec<String>>) -> impl Responder {
	let mut ids: Vec<&str> = Vec::new();
	for id in photo_ids.iter() {
		ids.push(&id);
	}

	delete_photos(user.id, &ids)
}

/// Get info about a photo
pub async fn route_get_photo(session: Option<Session>, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	match Photo::get(photo_id) {
		Ok(photo_opt) => {
			match photo_opt {
				Some(photo) => {
					if photo.can_view(&session) {
						HttpResponse::Ok().json(photo)
					}
					else {
						create_unauthorized_response()
					}
				},
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Get the thumbnail of a photo as file
pub async fn route_download_photo_thumbnail(session: Option<Session>, req: HttpRequest) -> impl Responder {
	match req.match_info().get("photo_id") {
		Some(photo_id) => create_response_for_photo(photo_id, session, false, |photo| &photo.path_thumbnail).await,
		None => create_not_found_response()
	}
}

/// Get the preview (large thumbnail) of a photo as file
pub async fn route_download_photo_preview(session: Option<Session>, req: HttpRequest) -> impl Responder {
	match req.match_info().get("photo_id") {
		Some(photo_id) => create_response_for_photo(photo_id, session, false, |photo| &photo.path_preview).await,
		None => create_not_found_response()
	}
}

/// Get the original of a photo as file
pub async fn route_download_photo_original(session: Option<Session>, req: HttpRequest) -> impl Responder {
	match req.match_info().get("photo_id") {
		Some(photo_id) => create_response_for_photo(photo_id, session, true, |photo| &photo.path_original).await,
		None => create_not_found_response()
	}
}

/// Upload a photo
pub async fn route_upload_photo(user: User, payload: Multipart) -> impl Responder {
	match get_form_data(payload).await {
		Ok(form_data) => {
			// let mut files_iter = form_data.first();//.iter(); //.filter(|d| d.name == "file");
			// let file_option = files_iter.next();
			// let remaining_files = files_iter.count();

			if form_data.len() > 1 {
				return create_bad_request_response(Box::from(UploadError::MoreThanOneFile));
			}

			let file_option = form_data.first();

			match file_option {
				Some(file) => {
					let photo = match &file.name.to_lowercase() {
						name if name.ends_with(".jpg")
							|| name.ends_with(".jpeg")
							|| name.ends_with(".png") =>
							Photo::parse_image_bytes(user.id, &file.bytes, "image/jpeg"),
						name if name.ends_with(".png") =>
							Photo::parse_image_bytes(user.id, &file.bytes, "image/png"),
						name if name.ends_with(".mp4") =>
							Photo::parse_mp4_bytes(user.id, &file.bytes),
						_ => Err(Box::from("Unsupported file type"))
					};

					match photo {
						Ok(photo) => {
							match photo.insert() {
								Ok(_) => create_created_response(&photo.id),
								Err(error) => create_bad_request_response(error)
							}
						},
						Err(error) => create_bad_request_response(error)
					}
				},
				None => create_bad_request_response(Box::from(UploadError::NoFile))
			}
		},
		Err(error) => create_bad_request_response(error)
	}
}


/// Delete multiple photos from database and disk
pub fn delete_photos(user_id: String, ids: &[&str]) -> impl Responder {
	// Check if all ids to be deleted are owned by user_id
	for id in ids {
		match Photo::get(id) {
			Ok(photo_opt) => {
				if let Some(photo) = photo_opt {
					if photo.user_id != user_id {
						return create_unauthorized_response();
					}
				}
			},
			Err(error) => return create_internal_server_error_response(Some(error))
		}
	}

	// Remove references to these photos from albums
	if let Err(error) = database::get_database().remove_photos_from_all_albums(ids) {
		return create_internal_server_error_response(Some(error));
	}
	if let Err(error) = database::get_database().remove_thumbs_from_all_albums(ids) {
		return create_internal_server_error_response(Some(error));
	}

	// Delete physical files for photo
	for id in ids {
		let result = delete_photo_files(&id);
		match result {
			Ok(_) => {},
			Err(error) => return create_internal_server_error_response(Some(error))
		}
	}

	// Delete all photos from database
	match database::get_database().delete_many(database::COLLECTION_PHOTOS, ids) {
		Ok(_) => create_ok_response(),
		Err(_) => create_not_found_response()
	}
}

/// Get the HTTP response that returns a photo from disk by its id.
/// Given user must have access to it.
async fn create_response_for_photo(photo_id: &str, session: Option<Session>, offer_as_download: bool, select_path: fn(&Photo) -> &str) -> actix_http::Response {
	match Photo::get(photo_id) {
		Ok(photo) => {
			match photo {
				Some(photo) => {
					if photo.can_view(&session) {
						serve_photo(&select_path(&photo), &photo.name, &photo.content_type, offer_as_download).await
					}
					else {
						create_unauthorized_response()
					}
				},
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Create an HTTP response that offers photo file at given path as download
async fn serve_photo(path: &str, file_name: &str, content_type: &str, offer_as_download: bool) -> actix_http::Response {
	match storage::get_file(path).await {
		Ok(file_bytes_option) => {
			match file_bytes_option {
				Some(file_bytes) => {
					HttpResponse::Ok()
						.content_type(content_type)
						.header(http::header::CONTENT_DISPOSITION,
							if offer_as_download {
								format!("attachment; filename=\"{}\"", file_name)
							} else {
								"inline;".to_string()
							})
						.body(file_bytes)
				},
				None => create_internal_server_error_response(Some(Box::from(FileError::NotFound)))
			}
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Deletes all physical files of a photo from file system
/// Original, thumbnail and preview images.
fn delete_photo_files(photo_id: &str) -> Result<()> {
	if let Some(photo) = Photo::get(&photo_id)? {
		storage::delete_file(&photo.path_original)?;
		storage::delete_file(&photo.path_preview)?;
		storage::delete_file(&photo.path_thumbnail)?;
	}
	Ok(())
}
