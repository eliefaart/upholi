use crate::database::models::session::Session;
use crate::database::models::share::DbShare;
use crate::database::models::user::User;
use crate::database::models::AccessControl;
use crate::database::{DatabaseEntity, DatabaseEntityUserOwned};
use crate::error::HttpError;
use crate::web::http::*;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use upholi_lib::http::request::FindSharesFilter;
use upholi_lib::models::{EncryptedShare, EncryptedShareUpsert};

/// Get all shares
pub async fn route_get_shares(user: User, filters: web::Query<FindSharesFilter>) -> Result<HttpResponse> {
	let filters = filters.into_inner();
	let shares: Vec<EncryptedShare> = DbShare::find_shares(&user.id, filters)
		.await
		.map_err(ErrorInternalServerError)?
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
		.ok_or_else(|| ErrorNotFound(HttpError::NotFound))?;

	if share.can_view(&session, None) {
		let share: EncryptedShare = share.into();
		Ok(HttpResponse::Ok().json(share))
	} else {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	}
}

/// Create a new share
pub async fn route_create_share(user: User, share: web::Json<EncryptedShareUpsert>) -> Result<HttpResponse> {
	let share = DbShare::from(share.into_inner(), &user.id);

	share.insert().await.map_err(ErrorInternalServerError)?;

	Ok(create_created_response(&share.id))
}

/// Update a share
pub async fn route_update_share(
	session: Session,
	req: HttpRequest,
	updated_share: web::Json<EncryptedShareUpsert>,
) -> Result<HttpResponse> {
	let share_id = req.match_info().get("share_id").unwrap();
	let updated_share = updated_share.into_inner();

	let user_id = session.user_id.clone().ok_or_else(|| ErrorUnauthorized(HttpError::Unauthorized))?;
	let mut share = DbShare::get_for_user(share_id, user_id.to_string())
		.await
		.map_err(|_| ErrorUnauthorized(HttpError::Unauthorized))?
		.ok_or_else(|| ErrorNotFound(HttpError::NotFound))?;

	if !share.can_update(&Some(session)) {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	} else {
		share.entity.type_ = updated_share.type_;
		share.entity.data = updated_share.data;
		share.entity.key = updated_share.key;
		share.entity.password = updated_share.password;
		share.entity.identifier_hash = updated_share.identifier_hash;

		share.update().await.map_err(ErrorInternalServerError)?;
		Ok(create_ok_response())
	}
}

/// Delete a share
pub async fn route_delete_share(session: Session, req: HttpRequest) -> Result<HttpResponse> {
	let share_id = req.match_info().get("share_id").unwrap();

	let user_id = session.user_id.clone().ok_or_else(|| ErrorUnauthorized(HttpError::Unauthorized))?;
	let share = DbShare::get_for_user(share_id, user_id.to_string())
		.await
		.map_err(ErrorInternalServerError)?
		.ok_or_else(|| ErrorUnauthorized(HttpError::Unauthorized))?;

	if !share.can_delete(&Some(session)) {
		Err(ErrorUnauthorized(HttpError::Unauthorized))
	} else {
		share.delete().await.map_err(ErrorInternalServerError)?;
		Ok(create_ok_response())
	}
}
