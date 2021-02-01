use actix_web::{web, HttpResponse, Responder};

use crate::{database::DatabaseEntity, web::cookies::create_session_cookie};
use crate::web::oauth2;
use crate::web::oauth2::OAuth2Provider;
use crate::web::http::*;
use crate::entities::user::User;
use crate::entities::session::Session;
use crate::web::handlers::requests::*;

/// OAuth: start login flow with an identity provider
pub async fn oauth_start_login(session_opt: Option<Session>) -> impl Responder {
	let request_has_session = session_opt.is_some();

	match get_session_or_create_new(session_opt) {
		Ok(mut session) => {
			match oauth2::get_provider().get_auth_url() {
				Ok(url_info) => {
					session.set_oauth_data(&url_info.csrf_token, &url_info.pkce_verifier);
					match session.update() {
						Ok(_) => {
							let mut response = HttpResponse::Found()
								.header(http::header::LOCATION, url_info.auth_url)
								.finish();

							// Append the new session cookie to the response
							if !request_has_session {
								let cookie = create_session_cookie(&session);

								if let Err(error) = response.add_cookie(&cookie) {
									return create_internal_server_error_response(Some(Box::new(error)));
								}
							}

							response
						},
						Err(error) => create_internal_server_error_response(Some(error))
					}
				},
				Err(error) => create_internal_server_error_response(Some(error))
			}
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// OAuth callback
pub async fn oauth_callback(mut session: Session, oauth_info: web::Query<OauthCallback>) -> impl Responder {
	match &session.oauth {
		Some(oauth_data) => {
			// Verify state value
			if oauth_data.state != oauth_info.state {
				println!("Invalid oauth state provided");
				return create_unauthorized_response();
			}

			// Verify code externally
			match oauth2::get_provider().get_access_token(&oauth_info.code, &oauth_data.pkce_verifier) {
				Ok(access_token) => {
					match oauth2::get_provider().get_user_info(&access_token).await {
						Ok(user_info) => {
							// Assign the user to the session, and clear oauth login data/tokens
							session.set_user(&user_info.id);
							session.oauth = None;

							match session.update() {
								Ok(_) => {
									// Redirect to home page
									HttpResponse::Found()
										.header(http::header::LOCATION, "/")
										.finish()
								},
								Err(error) => create_internal_server_error_response(Some(error))
							}
						},
						Err(error) => {
							create_internal_server_error_response(Some(error))
						}
					}
				},
				Err(error) => {
					println!("{}", error);
					create_unauthorized_response()
				}
			}
		},
		None => create_unauthorized_response()
	}
}

/// OAuth get info of current user
pub async fn oauth_user_info(user: User) -> impl Responder {
	HttpResponse::Ok().json(user)
}