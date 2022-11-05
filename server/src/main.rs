use crate::settings::Settings;
use anyhow::Result;
use axum::{
	async_trait,
	extract::{FromRequest, RequestParts},
	http::{HeaderMap, HeaderValue, StatusCode},
	middleware::Next,
	response::{IntoResponse, Response},
	routing::{get, get_service, post},
	Router,
};
use cookie::time::{Duration, OffsetDateTime};
use handlers::{files::*, items::*, shares::*, user::*};
use lazy_static::lazy_static;
use model::Session;
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer};
use tower_http::services::ServeDir;

mod database;
mod handlers;
mod model;
mod settings;
mod storage;

lazy_static! {
	/// Global application settings
	#[derive(Debug)]
	pub static ref SETTINGS: Settings = Settings::new();
}

const AUTH_COOKIE_NAME: &str = "session";
const AUTH_COOKIE_EXPIRATION_TIME_DAYS: i64 = 1;
const AUTH_COOKIE_SECURE: bool = false;

pub struct UserId(String);
pub struct OptionalSession(Option<Session>);

#[tokio::main]
async fn main() {
	let virtual_page_paths: [&str; 6] = ["/login", "/register", "/albums", "/album/", "/shared", "/s/"];
	let mut index_file_router = Router::new();

	for path in virtual_page_paths {
		index_file_router = index_file_router.nest(
			path,
			get_service(tower_http::services::ServeFile::new(format!(
				"{}/index.html",
				SETTINGS.server.wwwroot_path
			)))
			.handle_error(on_io_error),
		);
	}

	let app = Router::new()
		.route("/user", get(get_user).post(create_user))
		.route("/user/auth", post(authenticate_user))
		.route("/share", post(create_share))
		.route("/share/:id", get(get_share).delete(delete_share))
		.route("/share/:id/auth", get(is_authorized_for_share).post(authorize_share))
		.route("/text", get(get_text_keys))
		.route("/texts/:key_prefix", get(get_text_multiple))
		.route("/text/:key", get(get_text).post(set_text).delete(delete_text))
		.route("/file/:key", get(get_file).delete(delete_file))
		.route("/file", get(get_file_keys).post(set_files))
		.merge(index_file_router)
		.fallback(get_service(ServeDir::new(&SETTINGS.server.wwwroot_path)).handle_error(on_io_error))
		.layer(CookieManagerLayer::new())
		.layer(axum::middleware::from_fn(extend_session_cookie));

	// run it
	let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
	println!("listening on {}", addr);
	axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

#[async_trait]
impl<B> FromRequest<B> for UserId
where
	B: Send,
{
	type Rejection = StatusCode;

	async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
		let session = get_session_from_request(req).await?;
		let user_id = session.ok_or(StatusCode::UNAUTHORIZED)?.user_id.ok_or(StatusCode::UNAUTHORIZED)?;
		Ok(UserId(user_id))
	}
}

#[async_trait]
impl<B> FromRequest<B> for Session
where
	B: Send,
{
	type Rejection = StatusCode;

	async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
		let session = get_session_from_request(req).await?.ok_or(StatusCode::UNAUTHORIZED)?;
		Ok(session)
	}
}

#[async_trait]
impl<B> FromRequest<B> for OptionalSession
where
	B: Send,
{
	type Rejection = StatusCode;

	async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
		let session = get_session_from_request(req).await?;
		Ok(OptionalSession(session))
	}
}

async fn get_session_from_request<B: Send>(req: &mut RequestParts<B>) -> Result<Option<Session>, StatusCode> {
	let request_session_id = get_session_id_from_headers(req.headers()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	match request_session_id {
		Some(session_id) => {
			let session = database::get_session(&session_id)
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

			Ok(session)
		}
		None => Ok(None),
	}
}

async fn on_io_error(_err: std::io::Error) -> impl IntoResponse {
	StatusCode::INTERNAL_SERVER_ERROR
}

/// Middleware that extends the cookie duration by including it in the response headers if it is not present yet
async fn extend_session_cookie<B>(req: axum::http::Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
	let request_session_id = get_session_id_from_headers(req.headers()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	let mut response = next.run(req).await;

	if !response.headers().contains_key(axum::http::header::COOKIE) {
		if let Some(session_id) = request_session_id {
			let response_cookie = create_sesson_cookie(session_id.into());
			response.headers_mut().insert(
				axum::http::header::SET_COOKIE,
				HeaderValue::from_str(&response_cookie.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
			);
		}
	}

	Ok(response)
}

fn get_session_id_from_headers(headers: &HeaderMap) -> Result<Option<String>> {
	if let Some(cookie_header) = headers.get(axum::http::header::COOKIE).cloned() {
		let cookie_header = cookie_header.to_str()?;
		let session_cookie = Cookie::parse(cookie_header)?;
		let session_id = session_cookie.value();

		Ok(Some(session_id.into()))
	} else {
		Ok(None)
	}
}

pub fn create_sesson_cookie<'a>(session_id: String) -> Cookie<'a> {
	let mut expires_on = OffsetDateTime::now_utc();
	expires_on += Duration::days(AUTH_COOKIE_EXPIRATION_TIME_DAYS);
	Cookie::build(AUTH_COOKIE_NAME, session_id.to_owned())
		.path("/")
		.http_only(true)
		.secure(AUTH_COOKIE_SECURE)
		.expires(expires_on)
		.finish()
}
