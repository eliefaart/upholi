use actix_web::{HttpResponse, Responder, web};
use upholi_lib::http::{request::{Login, Register}};

use crate::{database::DatabaseEntity, entities::{session::Session, user::User}, web::{cookies::create_session_cookie, http::{create_internal_server_error_response, create_not_found_response, create_ok_response, create_unauthorized_response, get_session_or_create_new}}};

pub async fn route_register_user(info: web::Json<Register>) -> impl Responder {
	let info = info.into_inner();
	match User::create(info.username, info.password, info.key).await {
		Ok(_) => {
			create_ok_response()
		}
		Err(error) => {
			create_internal_server_error_response(Some(error))
		}
	}
}

pub async fn route_login_user(session: Option<Session>, info: web::Json<Login>) -> impl Responder {
	let info = info.into_inner();

	match User::get_by_username(&info.username).await {
		Ok(user) => {
			match user {
				Some(user) => {
					if user.password_valid(&info.password) {
						match get_session_or_create_new(session) {
							Ok(mut session) => {
								session.set_user(&user.id);
								match session.update() {
									Ok(_) => {
										let mut response = HttpResponse::Ok().json(user);
										let cookie = create_session_cookie(&session);

										match response.add_cookie(&cookie) {
											Ok(_) => response,
											Err(error) => create_internal_server_error_response(Some(Box::new(error)))
										}
									},
									Err(error) => create_internal_server_error_response(Some(error))
								}
							},
							Err(error) => create_internal_server_error_response(Some(error))
						}
					}
					else {
						create_unauthorized_response()
					}
				},
				None => create_not_found_response()
			}
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}


pub async fn route_user_info(user: User) -> impl Responder {
 	HttpResponse::Ok().json(user)
}