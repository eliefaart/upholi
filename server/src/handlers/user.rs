use super::auth_user_for_session;
use crate::database::*;
use crate::model::{Session, User};
use crate::storage::init_storage_for_user;
use crate::UserId;
use anyhow::{anyhow, Result};
use axum::{http::StatusCode, response::IntoResponse, Json};
use upholi_lib::http::{request::*, response::*};
use upholi_lib::ids::id;
use upholi_lib::passwords::{hash_password, verify_password_hash};

pub async fn get_user(UserId(_): UserId) -> StatusCode {
	StatusCode::OK
}

pub async fn create_user(session: Session, Json(user_info): Json<CreateUserRequest>) -> Result<impl IntoResponse, StatusCode> {
	let result = handler_create_user(&user_info)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	auth_user_for_session(session, &result.id)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok((StatusCode::CREATED, Json(result)))
}

async fn handler_create_user(user_info: &CreateUserRequest) -> Result<CreatedResult> {
	if get_user_by_username(&user_info.username).await?.is_some() {
		Err(anyhow!("A user with this username already exists."))
	} else {
		let password_phc = hash_password(&user_info.password)?;
		let user_id = id();
		let user = User {
			id: user_id.clone(),
			username: user_info.username.clone(),
			password_phc,
		};
		insert_user(&user).await?;
		init_storage_for_user(&user).await?;
		Ok(CreatedResult { id: user_id })
	}
}

pub async fn authenticate_user(session: Session, Json(credentials): Json<AuthenticateUserRequest>) -> impl IntoResponse {
	let user = get_user_by_username(&credentials.username)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
		.ok_or(StatusCode::NOT_FOUND)?;

	let password_correct = verify_password_hash(&credentials.password, &user.password_phc);
	if password_correct {
		auth_user_for_session(session, &user.id)
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
		Ok(StatusCode::OK)
	} else {
		Err(StatusCode::UNAUTHORIZED)
	}
}
