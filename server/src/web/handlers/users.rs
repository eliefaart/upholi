use crate::{
	database::{
		models::{session::Session, user::User},
		DatabaseEntity,
	},
	error::{LoginError, RegisterError},
	web::{
		cookies::create_session_cookie,
		http::{create_ok_response, get_session_or_create_new},
	},
};
use actix_web::error::{ErrorBadRequest, ErrorInternalServerError};
use actix_web::{web, HttpResponse, Result};
use upholi_lib::http::{
	request::{Login, Register},
	response::UserInfo,
};

pub async fn route_register_user(info: web::Json<Register>) -> Result<HttpResponse> {
	let info = info.into_inner();

	let username_exists = User::get_by_username(&info.username).await?.is_some();

	if username_exists {
		Err(ErrorInternalServerError(RegisterError::UsernameExists))
	} else if info.username.is_empty() {
		Err(ErrorInternalServerError(RegisterError::UsernameEmpty))
	} else if info.password.len() < crate::SETTINGS.users.password_min_length {
		Err(ErrorBadRequest(RegisterError::PasswordTooShort))
	} else {
		User::create(info.username, info.password, info.key)
			.await
			.map_err(ErrorInternalServerError)?;
		Ok(create_ok_response())
	}
}

pub async fn route_login_user(session: Option<Session>, info: web::Json<Login>) -> Result<HttpResponse> {
	let info = info.into_inner();

	let user = User::get_by_username(&info.username)
		.await
		.map_err(ErrorInternalServerError)?
		.ok_or_else(|| ErrorBadRequest(LoginError::InvalidCredentials))?;

	if user.password_valid(&info.password) {
		let mut session = get_session_or_create_new(session).await.map_err(ErrorInternalServerError)?;

		session.set_user(&user.id);
		session.update().await.map_err(ErrorInternalServerError)?;
		let mut response = HttpResponse::Ok().json(user);
		let cookie = create_session_cookie(&session);

		response.add_cookie(&cookie).map_err(ErrorInternalServerError)?;

		Ok(response)
	} else {
		Err(ErrorBadRequest(LoginError::InvalidCredentials))
	}
}

pub async fn route_user_info(user: User) -> Result<HttpResponse> {
	let user_info: UserInfo = user.into();
	Ok(HttpResponse::Ok().json(user_info))
}
