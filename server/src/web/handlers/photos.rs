use crate::entities::photo::Photo;
use crate::{entities::session::Session};
use actix_web::{HttpRequest, HttpResponse, Responder, web};
use actix_multipart::Multipart;
use upholi_lib::http::request::{CheckPhotoExists, EntityAuthorizationProof, RequestedEntity};
use upholi_lib::{PhotoVariant, http::*};

use crate::error::*;
use crate::database;
use crate::database::{Database, DatabaseExt, DatabaseEntity};
use crate::web::http::*;
use crate::storage;
use crate::entities::AccessControl;
use crate::entities::user::User;


/// Get all photos
pub async fn route_get_photos(user: User) -> impl Responder {
	match database::get_database().get_photos_for_user(&user.id) {
		Ok(photos) => HttpResponse::Ok().json(photos),
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Retreive 1..n requested photos.
pub async fn route_find_photos(_user: Option<User>, requested_photos: web::Json<Vec<RequestedEntity>>) -> impl Responder {
	let requested_photos = requested_photos.into_inner();

	// TODO: If no user, then proof for each photo must be present.. or something
	// Either way function feels weird still.

	match database::get_database().get_photos(requested_photos) {
		Ok(photos) => HttpResponse::Ok().json(photos),
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Get photo
pub async fn route_get_photo(session: Option<Session>, req: HttpRequest, proof: Option<web::Query<EntityAuthorizationProof>>) -> impl Responder {
	let proof = match proof {
		Some(proof) => Some(proof.into_inner()),
		None => None
	};

	match req.match_info().get("photo_id") {
		Some(photo_id) => {
			match Photo::get(&photo_id) {
				Ok(photo) => {
					match photo {
						Some(photo) =>{
							if photo.can_view(&session, proof) {
								HttpResponse::Ok().json(photo)
							}
							else {
								create_unauthorized_response()
							}
						},
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

/// Check if a photo exists for user by hash
pub async fn route_check_photo_exists(user: User, check: web::Query<CheckPhotoExists>) -> impl Responder {
	match Photo::hash_exists_for_user(&user.id, &check.hash) {
		Ok(exists) => {
			if exists {
				HttpResponse::NoContent().finish()
			}
			else {
				HttpResponse::NotFound().finish()
			}
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Uploads a photo
pub async fn route_upload_photo(user: User, payload: Multipart) -> impl Responder {
	match get_form_data(payload).await {
		Ok(form_data) => {
			let mut bytes_data: Vec<u8> = vec!{};
			let mut bytes_thumbnail: Vec<u8> = vec!{};
			let mut bytes_preview: Vec<u8> = vec!{};
			let mut data_original: Vec<u8> = vec!{};
			for part in form_data {
				match part.name.as_str() {
					"data" => bytes_data = part.bytes,
					"thumbnail" => bytes_thumbnail = part.bytes,
					"preview" => bytes_preview = part.bytes,
					"original" => data_original = part.bytes,
					_ => return create_bad_request_response(Box::from(UploadError::UnsupportedMultipartName))
				}
			}

			 match serde_json::from_slice::<request::UploadPhoto>(&bytes_data) {
				 Ok(photo) => {
					let mut db_photo: Photo = photo.into();
					db_photo.user_id = user.id.clone();

					let file_id_thumbnail = format!("{}-thumbnail", &db_photo.id);
					let file_id_preview = format!("{}-preview", &db_photo.id);
					let file_id_original = format!("{}-original", &db_photo.id);

					// Insert photo into DB
					if let Err(error) = db_photo.insert() {
						return create_internal_server_error_response(Some(error));
					}

					// Store photo bytes
					if let Err(error) = storage::store_file(&file_id_thumbnail, &user.id, &bytes_thumbnail).await {
						return create_internal_server_error_response(Some(error));
					}
					if let Err(error) = storage::store_file(&file_id_preview, &user.id, &bytes_preview).await {
						return create_internal_server_error_response(Some(error));
					}
					if let Err(error) = storage::store_file(&file_id_original, &user.id, &data_original).await {
						return create_internal_server_error_response(Some(error));
					}

					HttpResponse::Created().json(response::UploadPhoto {
						id: db_photo.id
					})
				 },
				 Err(error) => create_bad_request_response(Box::from(format!("{:?}", error)))
			 }
		},
		Err(error) => create_bad_request_response(error)
	}
}

/// Get the thumbnail of a photo as file
pub async fn route_download_photo_thumbnail(session: Option<Session>, req: HttpRequest, proof: Option<web::Query<EntityAuthorizationProof>>) -> impl Responder {
	download_photo(session, req, &PhotoVariant::Thumbnail, proof).await
}

/// Get the preview (large thumbnail) of a photo as file
pub async fn route_download_photo_preview(session: Option<Session>, req: HttpRequest, proof: Option<web::Query<EntityAuthorizationProof>>) -> impl Responder {
	download_photo(session, req, &PhotoVariant::Preview, proof).await
}

/// Get the original of a photo as file
pub async fn route_download_photo_original(session: Option<Session>, req: HttpRequest, proof: Option<web::Query<EntityAuthorizationProof>>) -> impl Responder {
	download_photo(session, req, &PhotoVariant::Original, proof).await
}

async fn download_photo(session: Option<Session>, req: HttpRequest, photo_variant: &PhotoVariant, proof: Option<web::Query<EntityAuthorizationProof>>) -> impl Responder {
	let proof = match proof {
		Some(proof) => Some(proof.into_inner()),
		None => None
	};

	match req.match_info().get("photo_id") {
		Some(photo_id) => {
			match Photo::get(&photo_id) {
				Ok(photo) => {
					match photo {
						Some(photo) => {
							if !photo.can_view(&session, proof) {
								create_unauthorized_response()
							}
							else {
								let file_id = &format!("{}-{}", photo_id, &photo_variant.to_string());
								match storage::get_file(file_id, &photo.user_id).await {
									Ok(bytes) => {
										match bytes {
											Some(bytes) => HttpResponse::Ok().body(bytes),
											None => create_not_found_response()
										}
									},
									Err(error) => create_internal_server_error_response(Some(error))
								}
							}
						},
						None => create_not_found_response()
					}
				},
				Err(error) => create_internal_server_error_response(Some(error))
			}
		},
		None => create_bad_request_response(Box::from("Photo ID invalid or missing"))
	}
}

/// Delete multiple photos from database and disk
pub async fn delete_photos(user_id: String, ids: &[&str]) -> impl Responder {
	let mut photos: Vec<Photo> = Vec::new();

	// Check if all ids to be deleted are owned by user_id
	for id in ids {
		match Photo::get(id) {
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

/// Deletes all physical files of a photo from file system
/// Original, thumbnail and preview images.
async fn delete_photo_files(photo: &Photo) -> Result<()> {
	storage::delete_file(&format!("{}-{}", &photo.id, &PhotoVariant::Original.to_string()), &photo.user_id).await?;
	storage::delete_file(&format!("{}-{}", &photo.id, &PhotoVariant::Preview.to_string()), &photo.user_id).await?;
	storage::delete_file(&format!("{}-{}", &photo.id, &PhotoVariant::Thumbnail.to_string()), &photo.user_id).await?;
	Ok(())
}
