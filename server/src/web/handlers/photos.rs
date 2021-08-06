use crate::entities::photo_new;
use crate::{entities::session::Session};
use actix_web::web::Bytes;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_multipart::Multipart;
use upholi_lib::http::*;

use crate::error::*;

use crate::database;
use crate::database::{Database, DatabaseExt, DatabaseEntity, DatabaseUserEntity};
use crate::web::http::*;
use crate::storage;
use crate::entities::AccessControl;
use crate::entities::user::User;
use crate::entities::photo::PhotoOld;
use crate::web::handlers::responses::*;


/// Get all photos
pub async fn route_get_photos(user: User) -> impl Responder {
	match database::get_database().get_photos_for_user(&user.id) {
		Ok(photos) => HttpResponse::Ok().json(photos),
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Get photo
pub async fn route_get_photo(user: User, req: HttpRequest) -> impl Responder {
	match req.match_info().get("photo_id") {
		Some(photo_id) => {
			match photo_new::Photo::get(&photo_id) {
				Ok(photo) => {
					match photo {
						Some(photo) => HttpResponse::Ok().json(photo),
						None => create_not_found_response()
					}
				}
				Err(error) => create_internal_server_error_response(Some(error))
			}
		},
		None => create_not_found_response()
	}
}

/// Delete a single photo
pub async fn route_delete_photo(user: User, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	delete_photos(user.id, &[photo_id]).await
}

/// Create/register a new photo
pub async fn route_upload_photo_info(user: User, data: web::Json<request::UploadPhoto>) -> impl Responder {
	let mut db_photo: photo_new::Photo = data.into_inner().into();
	db_photo.user_id = user.id;

	match db_photo.insert() {
		Ok(_) => {
			HttpResponse::Created().json(response::UploadPhoto {
				id: db_photo.id
			})
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Get the thumbnail of a photo as file
pub async fn route_download_photo_thumbnail(user: User, req: HttpRequest) -> impl Responder {
	match req.match_info().get("photo_id") {
		Some(photo_id) => {

			let photo_id = &format!("{}-thumbnail", photo_id);

			match storage::get_file(photo_id, &user.id).await {
				Ok(bytes) => {
					match bytes {
						Some(bytes) => {
							HttpResponse::Ok()
								//.content_type("application/octet-stream")
								//.append_header((http::header::CONTENT_DISPOSITION,
								// 	if offer_as_download {
								// 		format!("attachment; filename=\"{}\"", &photo.name)
								// 	} else {
								// 		"inline;".to_string()
								// 	}
								// ))
								.body(bytes)
						},
						None => create_not_found_response()
					}
				},
				Err(error) => create_internal_server_error_response(Some(error))
			}
		},
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

pub async fn route_upload_photo_original(user: User, bytes: Bytes, req: HttpRequest) -> impl Responder {
	upload_photo(&user.id, bytes, req, "-original").await
}

pub async fn route_upload_photo_thumbnail_multipart(user: User, payload: Multipart, req: HttpRequest) -> impl Responder {
	let file_name_postfix = "-thumbnail";

	match req.match_info().get("photo_id") {
		Some(photo_id) => {
			match get_form_data(payload).await {
				Ok(form_data) => {
					if form_data.len() > 1 {
						create_bad_request_response(Box::from(UploadError::MoreThanOneFile))
					}
					else {
						match form_data.first() {
							Some(file) => {
								let file_id = &format!("{}{}", photo_id, file_name_postfix);
								match storage::store_file(file_id, &user.id, &file.bytes).await {
									Ok(_) => create_ok_response(),
									Err(error) => create_internal_server_error_response(Some(error))
								}
							}
							None => create_bad_request_response(Box::from(UploadError::NoFile))
						}
					}
				},
				Err(error) => create_bad_request_response(error)
			}
		},
		None => create_not_found_response()
	}

}

pub async fn route_upload_photo_thumbnail(user: User, bytes: Bytes, req: HttpRequest) -> impl Responder {
	upload_photo(&user.id, bytes, req, "-thumbnail").await
}

pub async fn route_upload_photo_preview(user: User, bytes: Bytes, req: HttpRequest) -> impl Responder {
	upload_photo(&user.id, bytes, req, "-preview").await
}

async fn upload_photo(user_id: &str, bytes: Bytes, req: HttpRequest, file_name_postfix: &str) -> impl Responder {
	match req.match_info().get("photo_id") {
		Some(photo_id) => {
			let file_id = &format!("{}{}", photo_id, file_name_postfix);
			match storage::store_file(file_id, user_id, &bytes).await {
				Ok(_) => create_ok_response(),
				Err(error) => create_internal_server_error_response(Some(error))
			}
		},
		None => create_not_found_response()
	}
}






















/// Get all photos
// pub async fn route_get_photos(user: User) -> impl Responder {
// 	match Photo::get_all_as_user(user.id) {
// 		Ok(photos) => {
// 			let photos_small: Vec<PhotoSmall> = photos.into_iter()
// 				.map(PhotoSmall::from)
// 				.collect();
// 			HttpResponse::Ok().json(photos_small)
// 		},
// 		Err(error) => {
// 			println!("{}", error);
// 			create_internal_server_error_response(Some(error))
// 		}
// 	}
// }

// /// Delete a single photo
// pub async fn route_delete_photo(user: User, req: HttpRequest) -> impl Responder {
// 	let photo_id = req.match_info().get("photo_id").unwrap();

// 	delete_photos(user.id, &[photo_id]).await
// }

// /// Delete multiple photos
// pub async fn route_delete_photos(user: User, photo_ids: web::Json<Vec<String>>) -> impl Responder {
// 	let mut ids: Vec<&str> = Vec::new();
// 	for id in photo_ids.iter() {
// 		ids.push(&id);
// 	}

// 	delete_photos(user.id, &ids).await
// }

// /// Get info about a photo
// pub async fn route_get_photo(session: Option<Session>, req: HttpRequest) -> impl Responder {
// 	let photo_id = req.match_info().get("photo_id").unwrap();

// 	match Photo::get(photo_id) {
// 		Ok(photo_opt) => {
// 			match photo_opt {
// 				Some(photo) => {
// 					if photo.can_view(&session) {
// 						HttpResponse::Ok().json(photo)
// 					}
// 					else {
// 						create_unauthorized_response()
// 					}
// 				},
// 				None => create_not_found_response()
// 			}
// 		},
// 		Err(_) => create_unauthorized_response()
// 	}
// }

// /// Get the thumbnail of a photo as file
// pub async fn route_download_photo_thumbnail(session: Option<Session>, req: HttpRequest) -> impl Responder {
// 	match req.match_info().get("photo_id") {
// 		Some(photo_id) => create_response_for_photo(photo_id, session, false, |photo| &photo.path_thumbnail).await,
// 		None => create_not_found_response()
// 	}
// }

// /// Get the preview (large thumbnail) of a photo as file
// pub async fn route_download_photo_preview(session: Option<Session>, req: HttpRequest) -> impl Responder {
// 	match req.match_info().get("photo_id") {
// 		Some(photo_id) => create_response_for_photo(photo_id, session, false, |photo| &photo.path_preview).await,
// 		None => create_not_found_response()
// 	}
// }

// /// Get the original of a photo as file
// pub async fn route_download_photo_original(session: Option<Session>, req: HttpRequest) -> impl Responder {
// 	match req.match_info().get("photo_id") {
// 		Some(photo_id) => create_response_for_photo(photo_id, session, true, |photo| &photo.path_original).await,
// 		None => create_not_found_response()
// 	}
// }

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
							PhotoOld::parse_image_bytes(user.id, &file.bytes, "image/jpeg").await,
						name if name.ends_with(".png") =>
							PhotoOld::parse_image_bytes(user.id, &file.bytes, "image/png").await,
						name if name.ends_with(".mp4") =>
							PhotoOld::parse_mp4_bytes(user.id, &file.bytes).await,
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
pub async fn delete_photos(user_id: String, ids: &[&str]) -> impl Responder {
	let mut photos: Vec<photo_new::Photo> = Vec::new();

	// Check if all ids to be deleted are owned by user_id
	for id in ids {
		match photo_new::Photo::get(id) {
			Ok(photo) => {
				if let Some(photo) = photo {
					if photo.user_id != user_id {
						return create_unauthorized_response();
					}
					else {
						photos.push(photo);
					}
				}
			},
			Err(error) => return create_internal_server_error_response(Some(error))
		}
	}

	// Remove references to these photos from albums
	// if let Err(error) = database::get_database().remove_photos_from_all_albums(ids) {
	// 	return create_internal_server_error_response(Some(error));
	// }
	// if let Err(error) = database::get_database().remove_thumbs_from_all_albums(ids) {
	// 	return create_internal_server_error_response(Some(error));
	// }

	// Delete physical files for photo
	for photo in photos {
		let result = delete_photo_files(&photo).await;
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
async fn create_response_for_photo(photo_id: &str, session: Option<Session>, offer_as_download: bool, select_file_id: fn(&PhotoOld) -> &str) -> HttpResponse {
	match PhotoOld::get(photo_id) {
		Ok(photo) => {
			match photo {
				Some(photo) => {
					if photo.can_view(&session) {
						serve_photo(&select_file_id(&photo), &photo, offer_as_download).await
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
async fn serve_photo(file_id: &str, photo: &PhotoOld, offer_as_download: bool) -> HttpResponse {
	match storage::get_file(file_id, &photo.user_id).await {
		Ok(file) => {
			match file {
				Some(bytes) => {
					HttpResponse::Ok()
						.content_type(photo.content_type.to_owned())
						.append_header((http::header::CONTENT_DISPOSITION,
							if offer_as_download {
								format!("attachment; filename=\"{}\"", &photo.name)
							} else {
								"inline;".to_string()
							}
						))
						.body(bytes)
				},
				None => create_internal_server_error_response(Some(Box::from(FileError::NotFound)))
			}
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Deletes all physical files of a photo from file system
/// Original, thumbnail and preview images.
async fn delete_photo_files(photo: &photo_new::Photo) -> Result<()> {
	storage::delete_file(&format!("{}-original", &photo.id), &photo.user_id).await?;
	storage::delete_file(&format!("{}-preview", &photo.id), &photo.user_id).await?;
	storage::delete_file(&format!("{}-thumbnail", &photo.id), &photo.user_id).await?;
	Ok(())
}
