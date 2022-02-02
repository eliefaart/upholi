use crate::database::entities::album::Album;
use crate::database::entities::session::Session;
use crate::database::entities::user::User;
use crate::database::entities::AccessControl;
use crate::database::{DatabaseEntity, DatabaseUserEntity};
use crate::error::HttpError;
use crate::web::http::*;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use upholi_lib::http::request::{CreateAlbum, EntityAuthorizationProof};

/// Get all albums
pub async fn route_get_albums(user: User) -> Result<HttpResponse> {
	let albums = Album::get_all_as_user(user.id).await?;
	Ok(HttpResponse::Ok().json(albums))
}

/// Get extended information of an album
pub async fn route_get_album(
	session: Option<Session>,
	req: HttpRequest,
	proof: Option<web::Query<EntityAuthorizationProof>>,
) -> Result<HttpResponse> {
	let album_id = req.match_info().get("album_id").unwrap();
	let proof = proof.map(|proof| proof.into_inner());

	let album = Album::get(album_id)
		.await
		.map_err(|_| ErrorUnauthorized(HttpError::Unauthorized))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if album.can_view(&session, proof) {
		Ok(HttpResponse::Ok().json(album))
	} else {
		Ok(create_unauthorized_response())
	}
}

/// Create a new album
pub async fn route_create_album(user: User, album: web::Json<CreateAlbum>) -> Result<HttpResponse> {
	let mut album = Album::from(album.into_inner());
	album.user_id = user.id;

	album.insert().await?;
	Ok(create_created_response(&album.id))
}

/// Update an album
pub async fn route_update_album(session: Session, req: HttpRequest, updated_album: web::Json<CreateAlbum>) -> Result<HttpResponse> {
	let album_id = req.match_info().get("album_id").unwrap();
	let updated_album = updated_album.into_inner();

	let user_id = session.user_id.clone().ok_or(ErrorUnauthorized(HttpError::Unauthorized))?;
	let mut album = Album::get_as_user(album_id, user_id.to_string())
		.await
		.map_err(|_| ErrorUnauthorized(HttpError::Unauthorized))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if !album.can_update(&Some(session)) {
		Ok(create_unauthorized_response())
	} else {
		album.data = updated_album.data;
		album.key = updated_album.key;
		album.key_hash = updated_album.key_hash;

		album.update().await.map_err(|error| ErrorInternalServerError(error))?;
		Ok(create_ok_response())
	}
}

/// Delete an album
pub async fn route_delete_album(session: Session, req: HttpRequest) -> Result<HttpResponse> {
	let album_id = req.match_info().get("album_id").unwrap();

	let user_id = session.user_id.clone().ok_or(ErrorUnauthorized(HttpError::Unauthorized))?;
	let album = Album::get_as_user(album_id, user_id.to_string())
		.await
		.map_err(|_| ErrorUnauthorized(HttpError::Unauthorized))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if !album.can_delete(&Some(session)) {
		Ok(create_unauthorized_response())
	} else {
		album.delete().await.map_err(|error| ErrorInternalServerError(error))?;
		Ok(create_ok_response())
	}
}
