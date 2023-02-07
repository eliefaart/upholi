use crate::settings::Settings;
use anyhow::Result;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::{HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{delete, get, get_service, post},
    Router,
};
use cookie::{
    time::{Duration, OffsetDateTime},
    SameSite,
};
use database::upsert_session;
use handlers::{files::*, items::*, shares::*, user::*};
use lazy_static::lazy_static;
use model::Session;
use std::net::SocketAddr;
use tower_cookies::{Cookie, CookieManagerLayer};
use tower_http::services::ServeDir;
use upholi_lib::ids::id;

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

const SESSION_COOKIE_NAME: &str = "session";
const SESSION_COOKIE_EXPIRATION_TIME_DAYS: i64 = 30;

pub struct UserId(String);

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
        .route("/share/:id", delete(delete_share))
        .route("/share/:id/auth", get(is_authorized_for_share).post(authorize_share))
        .route("/item", get(get_item_ids).delete(delete_items))
        .route("/item/:id", get(get_item).post(set_item).delete(delete_item))
        .route("/file", get(get_file_ids).post(set_files).delete(delete_files))
        .route("/file/:id", get(get_file).delete(delete_file))
        .merge(index_file_router)
        .fallback(get_service(ServeDir::new(&SETTINGS.server.wwwroot_path)).handle_error(on_io_error))
        .layer(CookieManagerLayer::new())
        .layer(axum::middleware::from_fn(session_cookie_layer));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn on_io_error(_err: std::io::Error) -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

#[async_trait]
impl<B> FromRequest<B> for Session
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let session_id = get_session_id_from_headers(req.headers())
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        database::get_session(&session_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

#[async_trait]
impl<B> FromRequest<B> for UserId
where
    B: Send,
{
    type Rejection = StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let session = Session::from_request(req).await?;
        let user_id = session.user_id.ok_or(StatusCode::UNAUTHORIZED)?;
        Ok(UserId(user_id))
    }
}

/// Middleware that ensures a session exists, and extends its duration if a session was already present in the request.
async fn session_cookie_layer<B>(mut req: axum::http::Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let session_id = get_session_id_from_headers(req.headers()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let request_contains_session = session_id.is_some();

    // Create a new session if request did not contain one
    let session_id = match session_id {
        Some(session_id) => session_id,
        None => create_new_session().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    let request_is_secure = req.uri().scheme_str().unwrap_or("") == "https";
    let session_cookie = create_sesson_cookie(session_id, request_is_secure);

    // Add the newly created session to the request
    if !request_contains_session {
        let header_value = HeaderValue::from_str(&session_cookie.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        req.headers_mut().append(axum::http::header::COOKIE, header_value);
    }

    // Handle request
    let mut response = next.run(req).await;

    // Write the cookie to the response
    response.headers_mut().insert(
        axum::http::header::SET_COOKIE,
        HeaderValue::from_str(&session_cookie.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    );

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

async fn create_new_session() -> Result<String> {
    let session = Session {
        id: id(),
        user_id: None,
        shares: vec![],
    };
    upsert_session(&session).await?;
    Ok(session.id)
}

fn create_sesson_cookie<'a>(session_id: String, secure: bool) -> Cookie<'a> {
    let mut expires_on = OffsetDateTime::now_utc();
    expires_on += Duration::days(SESSION_COOKIE_EXPIRATION_TIME_DAYS);
    Cookie::build(SESSION_COOKIE_NAME, session_id)
        .path("/")
        .http_only(true)
        .secure(secure)
        .expires(expires_on)
        .same_site(SameSite::Strict)
        .finish()
}
