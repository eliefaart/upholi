use actix_web::{Error, HttpRequest, HttpResponse, FromRequest};
use actix_web::dev::Payload;
use actix_multipart::{Multipart, Field};
use actix_http::cookie::Cookie;
use serde::{Serialize, Deserialize};
use futures::{StreamExt, TryStreamExt};
use futures::future::{ok, err, Ready};

use crate::error::*;
use crate::entities::session::Session;
use crate::database::DatabaseEntity;
use crate::entities::user::User;

pub const SESSION_COOKIE_NAME: &str = "session";

pub struct FormData {
	pub name: String,
	pub bytes: Vec<u8>
}

/// Response data for HTTP 201 results
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatedResult {
	id: String
}

/// Response data for HTTP 4xx & 5xx results
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ErrorResult {
	message: String
}

/// Allow User to be used as function parameter for request handlers
impl FromRequest for User {
	type Error = Error;
	type Future = Ready<Result<User, Error>>;
	type Config = ();

	#[inline]
	fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
		match get_user_id(request) {
			Some(user_id) => ok(User{user_id}),
			None => err(Error::from(HttpResponse::Unauthorized()))
		}
	}
}

/// Allow Session to be used as function parameter for request handlers
impl FromRequest for Session {
	type Error = Error;
	type Future = Ready<Result<Session, Error>>;
	type Config = ();

	#[inline]
	fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
		match get_session(request) {
			Some(session) => ok(session),
			None => err(Error::from(HttpResponse::Unauthorized()))
		}
	}
}

/// Extract the session cookie from headers
pub fn get_session_cookie(req: &actix_web::http::header::HeaderMap) -> Option<Cookie> {
	// TODO: Look specifically for SESSION_COOKIE_NAME, among potentially multiple cookie headers
	let cookie_header = req.get("cookie")?;
	match cookie_header.to_str() {
		Ok(cookie_header_str) => {
			match Cookie::parse(cookie_header_str) {
				Ok(session_cookie) => Some(session_cookie),
				Err(_) => None
			}
		},
		Err(_) => None
	}
}

/// Extract Session from the HTTP request
fn get_session(req: &HttpRequest) -> Option<Session> {
	let session_cookie = get_session_cookie(req.headers())?;
	let session_id = session_cookie.value();
	match Session::get(&session_id) {
		Ok(session) => session,
		Err(_) => None
	}
}

/// Extract user_id from the HTTP request
fn get_user_id(req: &HttpRequest) -> Option<i64> {
	match get_session(req) {
		Some(session) => session.user_id,
		None => None
	}
}

/// Gets all fields from multipart payload.
pub async fn get_form_data(mut payload: Multipart) -> Result<Vec<FormData>> {
	let mut form_data: Vec<FormData> = Vec::new();

	while let Ok(Some(field)) = payload.try_next().await {

		match field.content_disposition() {
			Some(content_disposition) => {
				match content_disposition.get_name() {
					Some(key) => {
						let field_bytes = get_form_field_bytes(field).await?;
						form_data.push(FormData{
							name: key.to_string(),
							bytes: field_bytes
						});
					},
					None => return Err(Box::from(UploadError::HeaderContentDispositionInvalid))
				}
			},
			None => return Err(Box::from(UploadError::HeaderContentDispositionMissing))
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
			},
			Err(error) => return Err(Box::from(format!("{:?}", error)))
		}
	}

	Ok(field_bytes)
}

/// Create a HTTP 200 OK response
pub fn create_ok_response() -> actix_http::Response {
	HttpResponse::Ok().finish()
}

/// Create a HTTP 201 Created response
pub fn create_created_response(id: &str) -> actix_http::Response {
	HttpResponse::Created().json(CreatedResult{id: id.to_string()})
}

/// Create a HTTP 404 Not Found response
pub fn create_not_found_response() -> actix_http::Response {
	HttpResponse::NotFound().finish()
}

/// Create a HTTP 400 Bad Request response
pub fn create_bad_request_response(error: Box<dyn std::error::Error>) -> actix_http::Response {
	HttpResponse::BadRequest()
		.json(ErrorResult{
			message: format!("{:?}", error) 
		})
}

/// Create a HTTP 500 Internal Server Error response
pub fn create_internal_server_error_response(error: Option<Box<dyn std::error::Error>>) -> actix_http::Response {
	let mut response = HttpResponse::InternalServerError();

	match error {
		Some(error) => response.json(ErrorResult{
			message: format!("{:?}", error) 
		}),
		None => response.finish()
	}
}

/// Create a HTTP 501 Unauthorized response
pub fn create_unauthorized_response() -> actix_http::Response {
	HttpResponse::Unauthorized().finish()
}