use actix_web::{Error, HttpRequest, HttpResponse, Responder, FromRequest};
use actix_web::dev::Payload;
use actix_multipart::{Multipart, Field};
use actix_http::cookie::Cookie;
use serde::{Serialize, Deserialize};
use futures::{StreamExt, TryStreamExt};
use futures::future::{ok, err, Ready};

use crate::database;
use crate::session::{Session};
use crate::database::{DatabaseOperations, DatabaseUserOperations};
use crate::files;
use crate::photos;
use crate::photos::Photo;

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

/// Data associated with a session
#[derive(Debug, Serialize)]
pub struct User {
	pub user_id: i64
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
	Session::get(&session_id)
}

/// Extract user_id from the HTTP request
fn get_user_id(req: &HttpRequest) -> Option<i64> {
	match get_session(req) {
		Some(session) => session.user_id,
		None => None
	}
}

/// Gets all fields from multipart payload.
pub async fn get_form_data(mut payload: Multipart) -> Vec<FormData> {
	let mut form_data: Vec<FormData> = Vec::new();

	while let Ok(Some(field)) = payload.try_next().await {

		let content_disposition = field.content_disposition().unwrap();
		//let content_type = field.content_type();
		let key = content_disposition.get_name().unwrap();

		let field_bytes = get_form_field_bytes(field).await;
		form_data.push(FormData{
			name: key.to_string(),
			bytes: field_bytes
		});
	}

	form_data
}

/// Gets the bytes of a single multipart field.
async fn get_form_field_bytes(mut field: Field) -> Vec<u8> {
	let mut field_bytes: Vec<u8> = Vec::new();

	while let Some(chunk) = field.next().await {
		let chunk_bytes = chunk.unwrap();

		for byte in chunk_bytes {
			field_bytes.push(byte);
		}
	}

	field_bytes
}

/// Get the HTTP response that returns a photo from disk by its id.
/// As long as given user has access to it.
pub async fn download_photo(photo_id: &str, user_id: i64, select_path: fn(&Photo) -> &str) -> impl Responder {
	match Photo::get_as_user(photo_id, user_id) {
		Ok(photo_opt) => {
			match photo_opt {
				Some(photo_info) => {
					//serve_file(select_path(&photo_info), &photo_info.name),

					match files::get_photo(select_path(&photo_info)) {
						Some(file_bytes) => {
							HttpResponse::Ok()
								.content_type("image/jpeg")
								.header(http::header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", &photo_info.name))
								.body(file_bytes)
						},
						None => create_internal_server_error_response(Some("Error reading file content from disk, or file not found"))
					}
				},
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Delete multiple photos from database and disk
pub fn delete_photos(user_id: i64, ids: &[&str]) -> impl Responder {

	// Check if all ids to be deleted are owned by user_id
	for id in ids {
		match Photo::get(id) {
			Some(photo) => {
				if photo.user_id != user_id {
					return create_unauthorized_response();
				}
			},
			None => {}
		}
	}

	// Delete physical files for photo
	for id in ids {
		delete_photo_files(&id);
	}

	// Delete all photos from database
	match database::photo::delete_many(ids) {
		Ok(_) => create_ok_response(),
		Err(_) => create_not_found_response()
	}
}

/// Deletes all physical files of a photo from file system
/// Original, thumbnail and preview images.
fn delete_photo_files(photo_id: &str) {
	if let Some(photo) = photos::Photo::get(&photo_id) {
		files::delete_photo(&photo.path_original);
		files::delete_photo(&photo.path_preview);
		files::delete_photo(&photo.path_thumbnail);
	}
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
pub fn create_bad_request_response(message: &str) -> actix_http::Response {
	HttpResponse::BadRequest().json(ErrorResult{message: message.to_string()})
}

/// Create a HTTP 500 Internal Server Error response
pub fn create_internal_server_error_response(message: Option<&str>) -> actix_http::Response {
	let mut response = HttpResponse::InternalServerError();

	match message {
		Some(msg) => response.json(ErrorResult{message: msg.to_string()}),
		None => response.finish()
	}
}

/// Create a HTTP 501 Unauthorized response
pub fn create_unauthorized_response() -> actix_http::Response {
	HttpResponse::Unauthorized().finish()
}