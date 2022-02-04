use actix_multipart::{Field, Multipart};
use actix_web::cookie::Cookie;
use actix_web::dev::Payload;
use actix_web::error::{ErrorInternalServerError, ErrorNotFound, ErrorUnauthorized};
use actix_web::{Error, FromRequest, HttpRequest, HttpResponse};
use futures::{Future, StreamExt, TryStreamExt};
use std::pin::Pin;
use upholi_lib::http::response::CreatedResult;

use crate::database::entities::session::Session;
use crate::database::entities::user::User;
use crate::database::DatabaseEntity;
use crate::error::*;

pub const SESSION_COOKIE_NAME: &str = "session";

pub struct FormData {
	pub name: String,
	pub bytes: Vec<u8>,
}

/// Allow User to be used as function parameter for request handlers
impl FromRequest for User {
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

	#[inline]
	fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
		let headers = request.headers().clone();
		Box::pin(async move {
			match get_user_id(&headers).await {
				Some(user_id) => match User::get(&user_id).await {
					Ok(user_opt) => match user_opt {
						Some(user) => Ok(user),
						None => Err(ErrorNotFound("")),
					},
					Err(_) => Err(ErrorInternalServerError("")),
				},
				None => Err(ErrorUnauthorized("")),
			}
		})
	}
}

/// Allow Session to be used as function parameter for request handlers
impl FromRequest for Session {
	type Error = Error;
	type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

	#[inline]
	fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
		let headers = request.headers().clone();
		Box::pin(async move {
			match get_session(&headers).await {
				Some(session) => Ok(session),
				None => Err(ErrorUnauthorized("")),
			}
		})
	}
}

/// Extract the session cookie from headers
pub fn get_session_cookie(headers: &actix_web::http::header::HeaderMap) -> Option<Cookie> {
	// TODO: Look specifically for SESSION_COOKIE_NAME, among potentially multiple cookie headers
	let cookie_header = headers.get("cookie")?;
	match cookie_header.to_str() {
		Ok(cookie_header_str) => match Cookie::parse(cookie_header_str) {
			Ok(session_cookie) => Some(session_cookie),
			Err(_) => None,
		},
		Err(_) => None,
	}
}

/// Extract Session from the HTTP request
async fn get_session(headers: &actix_web::http::header::HeaderMap) -> Option<Session> {
	let session_cookie = get_session_cookie(headers)?;
	let session_id = session_cookie.value();
	match Session::get(session_id).await {
		Ok(session) => session,
		Err(_) => None,
	}
}

/// Extract user_id from the HTTP request
async fn get_user_id(headers: &actix_web::http::header::HeaderMap) -> Option<String> {
	match get_session(headers).await {
		Some(session) => session.user_id,
		None => None,
	}
}

/// Gets all fields from multipart payload.
pub async fn get_form_data(mut payload: Multipart) -> Result<Vec<FormData>> {
	let mut form_data = Vec::new();

	while let Ok(Some(field)) = payload.try_next().await {
		let content_disposition = field.content_disposition();

		match content_disposition.get_name() {
			Some(name) => {
				form_data.push(FormData {
					name: name.to_string(),
					bytes: get_form_field_bytes(field).await?,
				});
			}
			None => return Err(Box::from(UploadError::HeaderContentDispositionInvalid)),
		}
	}

	Ok(form_data)
}

/// Gets the bytes of a single multipart field.
async fn get_form_field_bytes(mut field: Field) -> Result<Vec<u8>> {
	let mut field_bytes: Vec<u8> = Vec::new();

	while let Some(chunk) = field.next().await {
		match chunk {
			Ok(chunk_bytes) => {
				for byte in chunk_bytes {
					field_bytes.push(byte);
				}
			}
			Err(error) => return Err(Box::from(format!("{:?}", error))),
		}
	}

	Ok(field_bytes)
}

pub async fn get_session_or_create_new(session_opt: Option<Session>) -> Result<Session> {
	let session: Session;

	// Create a new session if request didn't have one
	if let Some(existing_sesson) = session_opt {
		session = existing_sesson;
	} else {
		session = Session::new();
		session.insert().await?;
	}

	Ok(session)
}

/// Create a HTTP 200 OK response
pub fn create_ok_response() -> HttpResponse {
	HttpResponse::Ok().finish()
}

/// Create a HTTP 201 Created response
pub fn create_created_response(id: &str) -> HttpResponse {
	HttpResponse::Created().json(CreatedResult { id: id.to_string() })
}
