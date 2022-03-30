use crate::database::models::album::DbAlbum;
use crate::database::models::session::Session;
use crate::database::models::user::User;
use crate::database::models::AccessControl;
use crate::database::{DatabaseEntity, DatabaseEntityUserOwned};
use crate::error::HttpError;
use crate::web::http::*;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use upholi_lib::http::request::EntityAuthorizationProof;
use upholi_lib::models::{EncryptedAlbum, EncryptedAlbumUpsert};

/// Get all albums
pub async fn route_get_albums(user: User) -> Result<HttpResponse> {
	let db_albums = DbAlbum::get_all_for_user(user.id).await?;
	let albums: Vec<EncryptedAlbum> = db_albums.into_iter().map(|a| a.into()).collect();
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

	let db_album = DbAlbum::get(album_id)
		.await
		.map_err(|_| ErrorUnauthorized(HttpError::Unauthorized))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if db_album.can_view(&session, proof) {
		let album: EncryptedAlbum = db_album.into();
		Ok(HttpResponse::Ok().json(album))
	} else {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	}
}

/// Create a new album
pub async fn route_create_album(user: User, album: web::Json<EncryptedAlbumUpsert>) -> Result<HttpResponse> {
	let mut db_album = DbAlbum::from(album.into_inner(), &user.id);
	db_album.user_id = user.id;

	db_album.insert().await?;
	Ok(create_created_response(&db_album.id))
}

/// Update an album
pub async fn route_update_album(
	session: Session,
	req: HttpRequest,
	updated_album: web::Json<EncryptedAlbumUpsert>,
) -> Result<HttpResponse> {
	let album_id = req.match_info().get("album_id").unwrap();
	let updated_album = updated_album.into_inner();

	let user_id = session.user_id.clone().ok_or(ErrorUnauthorized(HttpError::Unauthorized))?;
	let mut db_album = DbAlbum::get_for_user(album_id, user_id.to_string())
		.await
		.map_err(|_| ErrorUnauthorized(HttpError::Unauthorized))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if !db_album.can_update(&Some(session)) {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	} else {
		db_album.entity.data = updated_album.data;
		db_album.entity.key = updated_album.key;
		db_album.entity.key_hash = updated_album.key_hash;

		db_album.update().await.map_err(|error| ErrorInternalServerError(error))?;
		Ok(create_ok_response())
	}
}

/// Delete an album
pub async fn route_delete_album(session: Session, req: HttpRequest) -> Result<HttpResponse> {
	let album_id = req.match_info().get("album_id").unwrap();

	let user_id = session.user_id.clone().ok_or(ErrorUnauthorized(HttpError::Unauthorized))?;
	let db_album = DbAlbum::get_for_user(album_id, user_id.to_string())
		.await
		.map_err(|_| ErrorUnauthorized(HttpError::Unauthorized))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if !db_album.can_delete(&Some(session)) {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	} else {
		db_album.delete().await.map_err(|error| ErrorInternalServerError(error))?;
		Ok(create_ok_response())
	}
}
