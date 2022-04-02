use crate::database::models::photo::DbPhoto;
use crate::database::models::session::Session;
use crate::database::models::user::User;
use crate::database::models::AccessControl;
use crate::database::{self, DatabaseEntity, DatabaseEntityBatch, DatabaseEntityMinimal, DatabaseEntityUserOwned};
use crate::error::{HttpError, UploadError};
use crate::storage;
use crate::web::http::*;
use actix_multipart::Multipart;
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use upholi_lib::http::request::{CheckExists, EntityAuthorizationProof, FindEntity};
use upholi_lib::http::response::{CheckExistsResult, CreatedResult};
use upholi_lib::models::{EncryptedPhoto, EncryptedPhotoUpsert};
use upholi_lib::PhotoVariant;

/// Get all photos
pub async fn route_get_photos(user: User) -> Result<HttpResponse> {
	let photos: Vec<EncryptedPhoto> = DbPhoto::get_all_for_user(user.id)
		.await
		.map_err(|error| ErrorInternalServerError(error))?
		.into_iter()
		.map(|p| p.into())
		.collect();

	Ok(HttpResponse::Ok().json(photos))
}

pub async fn route_get_photos_minimal(user: User) -> Result<HttpResponse> {
	let photos = DbPhoto::get_all_for_user_minimal(user.id)
		.await
		.map_err(|error| ErrorInternalServerError(error))?;

	Ok(HttpResponse::Ok().json(photos))
}

/// Retreive 1..n requested photos.
pub async fn route_find_photos(_user: Option<User>, requested_photos: web::Json<Vec<FindEntity>>) -> Result<HttpResponse> {
	let requested_photos = requested_photos.into_inner();

	// TODO: If no user, then proof for each photo must be present.. or something
	// Either way function feels weird still.

	let photos: Vec<EncryptedPhoto> = database::find_photos(requested_photos)
		.await
		.map_err(|error| ErrorInternalServerError(error))?
		.into_iter()
		.map(|p| p.into())
		.collect();

	Ok(HttpResponse::Ok().json(photos))
}

/// Retreive 1..n requested photos.
pub async fn route_find_photos_minimal(_user: Option<User>, requested_photos: web::Json<Vec<FindEntity>>) -> Result<HttpResponse> {
	let requested_photos = requested_photos.into_inner();
	let photos = database::find_photos_minimal(requested_photos)
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
	let photo = DbPhoto::get(photo_id)
		.await
		.map_err(|error| ErrorInternalServerError(error))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if photo.can_view(&session, proof) {
		let photo: EncryptedPhoto = photo.into();
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
pub async fn route_check_photo_exists(user: User, check: web::Query<CheckExists>) -> Result<HttpResponse> {
	let photo_id_option = DbPhoto::get_photo_id_for_hash(&user.id, &check.hash)
		.await
		.map_err(|error| ErrorInternalServerError(error))?;

	Ok(HttpResponse::Ok().json(CheckExistsResult {
		exists: photo_id_option.is_some(),
		found_id: photo_id_option,
	}))
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

	let photo = serde_json::from_slice::<EncryptedPhotoUpsert>(&bytes_data).map_err(|error| ErrorBadRequest(error))?;

	let mut db_photo = DbPhoto::from(photo, &user.id);
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

	Ok(HttpResponse::Created().json(CreatedResult { id: db_photo.id }))
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
	let photo = DbPhoto::get(photo_id)
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
	let mut photos: Vec<DbPhoto> = Vec::new();

	// Check if all ids to be deleted are owned by user_id
	for id in ids {
		let photo = DbPhoto::get(id).await.map_err(|error| ErrorInternalServerError(error))?;

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
	DbPhoto::delete_many(ids).await.map_err(|_| ErrorNotFound(HttpError::NotFound))?;
	Ok(create_ok_response())
}

/// Deletes all physical files of a photo from file system
/// Original, thumbnail and preview images.
async fn delete_photo_files(photo: &DbPhoto) -> Result<()> {
	storage::delete_file(&format!("{}-{}", &photo.id, &PhotoVariant::Original.to_string()), &photo.user_id).await?;
	storage::delete_file(&format!("{}-{}", &photo.id, &PhotoVariant::Preview.to_string()), &photo.user_id).await?;
	storage::delete_file(&format!("{}-{}", &photo.id, &PhotoVariant::Thumbnail.to_string()), &photo.user_id).await?;
	Ok(())
}
