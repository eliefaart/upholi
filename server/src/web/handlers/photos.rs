use actix_multipart::Multipart;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use upholi_lib::http::request::{CheckPhotoExists, EntityAuthorizationProof, RequestedEntity};
use upholi_lib::{http::*, PhotoVariant};

use crate::database::entities::photo::Photo;
use crate::database::entities::session::Session;
use crate::database::entities::user::User;
use crate::database::entities::AccessControl;
use crate::database::{self, DatabaseEntity};
use crate::error::{HttpError, UploadError};
use crate::storage;
use crate::web::http::*;

/// Get all photos
pub async fn route_get_photos(user: User) -> Result<HttpResponse> {
	let photos = database::get_photos_for_user(&user.id)
		.await
		.map_err(|error| ErrorInternalServerError(error))?;
	Ok(HttpResponse::Ok().json(photos))
}

/// Retreive 1..n requested photos.
pub async fn route_find_photos(_user: Option<User>, requested_photos: web::Json<Vec<RequestedEntity>>) -> Result<HttpResponse> {
	let requested_photos = requested_photos.into_inner();

	// TODO: If no user, then proof for each photo must be present.. or something
	// Either way function feels weird still.

	let photos = database::get_photos(requested_photos)
		.await
		.map_err(|error| ErrorInternalServerError(error))?;
	Ok(HttpResponse::Ok().json(photos))
}

/// Get photo
pub async fn route_get_photo(
	session: Option<Session>,
	req: HttpRequest,
	proof: Option<web::Query<EntityAuthorizationProof>>,
) -> Result<HttpResponse> {
	let proof = proof.map(|proof| proof.into_inner());

	let photo_id = req.match_info().get("photo_id").ok_or(ErrorNotFound(HttpError::NotFound))?;
	let photo = Photo::get(photo_id)
		.await
		.map_err(|error| ErrorInternalServerError(error))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if photo.can_view(&session, proof) {
		Ok(HttpResponse::Ok().json(photo))
	} else {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	}
}

/// Delete a single photo
pub async fn route_delete_photo(user: User, req: HttpRequest) -> Result<HttpResponse> {
	let photo_id = req.match_info().get("photo_id").unwrap();
	delete_photos(user.id, &[photo_id]).await
}

/// Check if a photo exists for user by hash
pub async fn route_check_photo_exists(user: User, check: web::Query<CheckPhotoExists>) -> Result<HttpResponse> {
	let exists = Photo::hash_exists_for_user(&user.id, &check.hash)
		.await
		.map_err(|error| ErrorInternalServerError(error))?;

	if exists {
		Ok(HttpResponse::NoContent().finish())
	} else {
		Ok(HttpResponse::NotFound().finish())
	}
}

/// Uploads a photo
pub async fn route_upload_photo(user: User, payload: Multipart) -> Result<HttpResponse> {
	let form_data = get_form_data(payload).await.map_err(|error| ErrorBadRequest(error))?;
	let mut bytes_data: Vec<u8> = vec![];
	let mut bytes_thumbnail: Vec<u8> = vec![];
	let mut bytes_preview: Vec<u8> = vec![];
	let mut data_original: Vec<u8> = vec![];
	for part in form_data {
		match part.name.as_str() {
			"data" => bytes_data = part.bytes,
			"thumbnail" => bytes_thumbnail = part.bytes,
			"preview" => bytes_preview = part.bytes,
			"original" => data_original = part.bytes,
			_ => return Err(ErrorBadRequest(UploadError::UnsupportedMultipartName)),
		}
	}

	let photo = serde_json::from_slice::<request::UploadPhoto>(&bytes_data).map_err(|error| ErrorBadRequest(error))?;

	let mut db_photo: Photo = photo.into();
	db_photo.user_id = user.id.clone();

	let file_id_thumbnail = format!("{}-thumbnail", &db_photo.id);
	let file_id_preview = format!("{}-preview", &db_photo.id);
	let file_id_original = format!("{}-original", &db_photo.id);

	// Insert photo into DB
	db_photo.insert().await.map_err(|error| ErrorInternalServerError(error))?;

	// Store photo bytes
	storage::store_file(&file_id_thumbnail, &user.id, &bytes_thumbnail)
		.await
		.map_err(|error| ErrorInternalServerError(error))?;
	storage::store_file(&file_id_preview, &user.id, &bytes_preview)
		.await
		.map_err(|error| ErrorInternalServerError(error))?;
	storage::store_file(&file_id_original, &user.id, &data_original)
		.await
		.map_err(|error| ErrorInternalServerError(error))?;

	Ok(HttpResponse::Created().json(response::UploadPhoto { id: db_photo.id }))
}

/// Get the thumbnail of a photo as file
pub async fn route_download_photo_thumbnail(
	session: Option<Session>,
	req: HttpRequest,
	proof: Option<web::Query<EntityAuthorizationProof>>,
) -> Result<HttpResponse> {
	download_photo(session, req, &PhotoVariant::Thumbnail, proof).await
}

/// Get the preview (large thumbnail) of a photo as file
pub async fn route_download_photo_preview(
	session: Option<Session>,
	req: HttpRequest,
	proof: Option<web::Query<EntityAuthorizationProof>>,
) -> Result<HttpResponse> {
	download_photo(session, req, &PhotoVariant::Preview, proof).await
}

/// Get the original of a photo as file
pub async fn route_download_photo_original(
	session: Option<Session>,
	req: HttpRequest,
	proof: Option<web::Query<EntityAuthorizationProof>>,
) -> Result<HttpResponse> {
	download_photo(session, req, &PhotoVariant::Original, proof).await
}

async fn download_photo(
	session: Option<Session>,
	req: HttpRequest,
	photo_variant: &PhotoVariant,
	proof: Option<web::Query<EntityAuthorizationProof>>,
) -> Result<HttpResponse> {
	let proof = proof.map(|proof| proof.into_inner());

	let photo_id = req
		.match_info()
		.get("photo_id")
		.ok_or(ErrorBadRequest("Photo ID invalid or missing"))?;
	let photo = Photo::get(photo_id)
		.await
		.map_err(|error| ErrorInternalServerError(error))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if !photo.can_view(&session, proof) {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	} else {
		let file_id = &format!("{}-{}", photo_id, &photo_variant.to_string());
		let bytes = storage::get_file(file_id, &photo.user_id)
			.await
			.map_err(|error| ErrorInternalServerError(error))?
			.ok_or(ErrorNotFound(HttpError::NotFound))?;
		Ok(HttpResponse::Ok().body(bytes))
	}
}

/// Delete multiple photos from database and disk
pub async fn delete_photos(user_id: String, ids: &[&str]) -> Result<HttpResponse> {
	let mut photos: Vec<Photo> = Vec::new();

	// Check if all ids to be deleted are owned by user_id
	for id in ids {
		let photo = Photo::get(id).await.map_err(|error| ErrorInternalServerError(error))?;

		if let Some(photo) = photo {
			if photo.user_id != user_id {
				return Err(ErrorUnauthorized(HttpError::Unauthorized));
			} else {
				photos.push(photo);
			}
		}
	}

	// Delete physical files for photo
	for photo in photos {
		delete_photo_files(&photo).await.map_err(|error| ErrorInternalServerError(error))?;
	}

	// Delete all photos from database
	database::delete_photos(ids).await.map_err(|_| ErrorNotFound(HttpError::NotFound))?;
	Ok(create_ok_response())
}

/// Deletes all physical files of a photo from file system
/// Original, thumbnail and preview images.
async fn delete_photo_files(photo: &Photo) -> Result<()> {
	storage::delete_file(&format!("{}-{}", &photo.id, &PhotoVariant::Original.to_string()), &photo.user_id).await?;
	storage::delete_file(&format!("{}-{}", &photo.id, &PhotoVariant::Preview.to_string()), &photo.user_id).await?;
	storage::delete_file(&format!("{}-{}", &photo.id, &PhotoVariant::Thumbnail.to_string()), &photo.user_id).await?;
	Ok(())
}
