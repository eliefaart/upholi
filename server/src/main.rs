use crate::settings::Settings;
use anyhow::Result;
use axum::{
    async_trait,
    extract::{DefaultBodyLimit, FromRequestParts},
    http::{request::Parts, HeaderMap, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
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
    let virtual_page_paths: [&str; 7] = ["/404", "/login", "/register", "/albums", "/album/", "/shared", "/s/"];
    let mut index_file_router = Router::new();

    for path in virtual_page_paths {
        index_file_router = index_file_router.nest_service(
            path,
            tower_http::services::ServeFile::new(format!("{}/index.html", SETTINGS.server.wwwroot_path)),
        );
    }

    let api_routes = Router::new()
        .route("/user", get(get_user).post(create_user))
        .route("/user/auth", post(authenticate_user))
        .route("/share", post(create_share))
        .route("/share/:id", delete(delete_share))
        .route("/share/:id/auth", get(is_authorized_for_share).post(authorize_share))
        .route("/item", get(get_item_ids).delete(delete_items))
        .route("/item/:id", get(get_item).post(set_item).delete(delete_item))
        .route(
            "/file",
            get(get_file_ids)
                .post(set_files)
                .layer(DefaultBodyLimit::max(52_428_800))
                .delete(delete_files),
        )
        .route("/file/:id", get(get_file).delete(delete_file));

    let app = Router::new()
        .nest("/api", api_routes)
        .merge(index_file_router)
        .fallback(get_service(ServeDir::new(&SETTINGS.server.wwwroot_path)))
        .layer(CookieManagerLayer::new())
        .layer(axum::middleware::from_fn(session_cookie_layer));

    // run it
    let addr = SETTINGS
        .server
        .address
        .parse()
        .unwrap_or_else(|_| panic!("Invalid server address: {}", SETTINGS.server.address));
    println!("listening on {addr}");
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

#[async_trait]
impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = StatusCode;
    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let session_id = get_session_id_from_headers(&parts.headers)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)?;

        database::get_session(&session_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = StatusCode;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state).await?;
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
        None => create_new_session()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    };

    let session_cookie = create_sesson_cookie(session_id);

    // Add the newly created session to the request
    if !request_contains_session {
        let header_value =
            HeaderValue::from_str(&session_cookie.to_string()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
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

fn create_sesson_cookie<'a>(session_id: String) -> Cookie<'a> {
    let mut expires_on = OffsetDateTime::now_utc();
    expires_on += Duration::days(SESSION_COOKIE_EXPIRATION_TIME_DAYS);
    Cookie::build(SESSION_COOKIE_NAME, session_id)
        .path("/")
        .http_only(true)
        .secure(true)
        .expires(expires_on)
        .same_site(SameSite::Strict)
        .finish()
}
