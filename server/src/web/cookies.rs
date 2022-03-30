use actix_web::cookie::Cookie;

use crate::database::models::session::Session;

use super::http::SESSION_COOKIE_NAME;

/// Create a cookie to store the session ID
pub fn create_session_cookie(session: &Session) -> Cookie {
	let mut cookie = Cookie::new(SESSION_COOKIE_NAME, session.id.to_string());
	cookie.set_secure(true);
	cookie.set_http_only(true);
	cookie.set_path("/");
	cookie.make_permanent();

	cookie
}
