use crate::{
	database::{
		entities::{session::Session, user::User},
		DatabaseEntity,
	},
	error::{LoginError, RegisterError},
	web::{
		cookies::create_session_cookie,
		http::{create_bad_request_response, create_internal_server_error_response, create_ok_response, get_session_or_create_new},
	},
};
use actix_web::{web, HttpResponse, Responder};
use upholi_lib::http::request::{Login, Register};

pub async fn route_register_user(info: web::Json<Register>) -> impl Responder {
	let info = info.into_inner();

	if info.username.is_empty() {
		create_bad_request_response(Box::from(RegisterError::UsernameEmpty))
	} else if info.password.len() < crate::SETTINGS.users.password_min_length {
		create_bad_request_response(Box::from(RegisterError::PasswordTooShort))
	} else {
		match User::create(info.username, info.password, info.key).await {
			Ok(_) => create_ok_response(),
			Err(error) => create_internal_server_error_response(Some(error)),
		}
	}
}

pub async fn route_login_user(session: Option<Session>, info: web::Json<Login>) -> impl Responder {
	let info = info.into_inner();

	match User::get_by_username(&info.username).await {
		Ok(user) => match user {
			Some(user) => {
				if user.password_valid(&info.password) {
					match get_session_or_create_new(session).await {
						Ok(mut session) => {
							session.set_user(&user.id);
							match session.update().await {
								Ok(_) => {
									let mut response = HttpResponse::Ok().json(user);
									let cookie = create_session_cookie(&session);

									match response.add_cookie(&cookie) {
										Ok(_) => response,
										Err(error) => create_internal_server_error_response(Some(Box::new(error))),
									}
								}
								Err(error) => create_internal_server_error_response(Some(error)),
							}
						}
						Err(error) => create_internal_server_error_response(Some(error)),
					}
				} else {
					create_bad_request_response(Box::from(LoginError::InvalidCredentials))
				}
			}
			None => create_bad_request_response(Box::from(LoginError::InvalidCredentials)),
		},
		Err(error) => create_internal_server_error_response(Some(error)),
	}
}

pub async fn route_user_info(user: User) -> impl Responder {
	HttpResponse::Ok().json(user)
}
