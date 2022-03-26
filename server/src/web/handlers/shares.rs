use crate::database::entities::session::Session;
use crate::database::entities::share::DbShare;
use crate::database::entities::user::User;
use crate::database::entities::AccessControl;
use crate::database::{DatabaseEntity, DatabaseEntityUserOwned};
use crate::error::HttpError;
use crate::web::http::*;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use upholi_lib::http::request::{FindSharesFilter, UpsertShare};
use upholi_lib::http::response::Share;

/// Get all shares
pub async fn route_get_shares(user: User, filters: web::Query<FindSharesFilter>) -> Result<HttpResponse> {
	let filters = filters.into_inner();
	let shares: Vec<Share> = DbShare::find_shares(&user.id, filters)
		.await
		.map_err(|error| ErrorInternalServerError(error))?
		.into_iter()
		.map(|s| s.into())
		.collect();
	Ok(HttpResponse::Ok().json(shares))
}

/// Get extended information of a share
pub async fn route_get_share(session: Option<Session>, req: HttpRequest) -> Result<HttpResponse> {
	let share_id = req.match_info().get("share_id").unwrap();

	let share = DbShare::get(share_id)
		.await
		.map_err(|_| ErrorUnauthorized(HttpError::Unauthorized))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if share.can_view(&session, None) {
		let share: Share = share.into();
		Ok(HttpResponse::Ok().json(share))
	} else {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	}
}

/// Create a new share
pub async fn route_create_share(user: User, share: web::Json<UpsertShare>) -> Result<HttpResponse> {
	let share = DbShare::from(share.into_inner(), &user.id);

	share.insert().await.map_err(|error| ErrorInternalServerError(error))?;

	Ok(create_created_response(&share.id))
}

/// Update a share
pub async fn route_update_share(session: Session, req: HttpRequest, updated_share: web::Json<UpsertShare>) -> Result<HttpResponse> {
	let share_id = req.match_info().get("share_id").unwrap();
	let updated_share = updated_share.into_inner();

	let user_id = session.user_id.clone().ok_or(ErrorUnauthorized(HttpError::Unauthorized))?;
	let mut share = DbShare::get_for_user(share_id, user_id.to_string())
		.await
		.map_err(|_| ErrorUnauthorized(HttpError::Unauthorized))?
		.ok_or(ErrorNotFound(HttpError::NotFound))?;

	if !share.can_update(&Some(session)) {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	} else {
		share.entity.type_ = updated_share.type_;
		share.entity.data = updated_share.data;
		share.entity.key = updated_share.key;
		share.entity.password = updated_share.password;
		share.entity.identifier_hash = updated_share.identifier_hash;

		share.update().await.map_err(|error| ErrorInternalServerError(error))?;
		Ok(create_ok_response())
	}
}

/// Delete a share
pub async fn route_delete_share(session: Session, req: HttpRequest) -> Result<HttpResponse> {
	let share_id = req.match_info().get("share_id").unwrap();

	let user_id = session.user_id.clone().ok_or(ErrorUnauthorized(HttpError::Unauthorized))?;
	let share = DbShare::get_for_user(share_id, user_id.to_string())
		.await
		.map_err(|error| ErrorInternalServerError(error))?
		.ok_or(ErrorUnauthorized(HttpError::Unauthorized))?;

	if !share.can_delete(&Some(session)) {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	} else {
		share.delete().await.map_err(|error| ErrorInternalServerError(error))?;
		Ok(create_ok_response())
	}
}
