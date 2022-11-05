use super::auth_share_for_session;
use crate::database::{self, *};
use crate::model::{Session, Share, TextItemData};
use crate::{OptionalSession, UserId};
use anyhow::Result;
use axum::{extract::Path, http::StatusCode, Json};
use tower_cookies::Cookies;
use upholi_lib::http::{request::*, response::*};
use upholi_lib::passwords::{hash_password, verify_password_hash};

pub async fn is_authorized_for_share(Path(id): Path<String>, session: Session) -> StatusCode {
	match session.shares.contains(&id) {
		true => StatusCode::OK,
		false => StatusCode::UNAUTHORIZED,
	}
}

pub async fn authorize_share(
	cookies: Cookies,
	session: OptionalSession,
	Path(id): Path<String>,
	Json(credentials): Json<AuthorizeShareRequest>,
) -> Result<StatusCode, StatusCode> {
	let session = session.0;
	let already_authorized = session.is_some() && session.as_ref().unwrap().shares.contains(&id);

	if already_authorized {
		// This session is already authorized to this share; we won't verify the provided password.
		Ok(StatusCode::OK)
	} else {
		let share = database::get_share(&id)
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
			.ok_or(StatusCode::NOT_FOUND)?;

		let password_correct = verify_password_hash(&credentials.password, &share.password_phc);
		if password_correct {
			auth_share_for_session(session, cookies, &share.id)
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
			Ok(StatusCode::OK)
		} else {
			Err(StatusCode::UNAUTHORIZED)
		}
	}
}

pub async fn get_share(Path(id): Path<String>) -> Result<Json<GetShareResult>, StatusCode> {
	let share = database::get_share(&id)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
		.ok_or(StatusCode::NOT_FOUND)?;

	Ok(Json(GetShareResult {
		base64: share.data.base64,
		nonce: share.data.nonce,
	}))
}

pub async fn create_share(UserId(user_id): UserId, Json(share): Json<UpsertShareRequest>) -> Result<StatusCode, StatusCode> {
	let password_phc = hash_password(&share.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	let item_ids_for_share = share.items;
	let share = Share {
		id: share.id.clone(),
		user_id,
		password_phc,
		data: TextItemData {
			base64: share.base64,
			nonce: share.nonce,
		},
	};

	upsert_share(&share).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	set_items_for_share(&share.id, &item_ids_for_share)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	remove_authorizations_for_share(&share.id)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	Ok(StatusCode::OK)
}

pub async fn delete_share(UserId(user_id): UserId, Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
	database::delete_share(&user_id, &id)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	remove_items_from_share(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(StatusCode::OK)
}
