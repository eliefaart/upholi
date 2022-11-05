use crate::create_sesson_cookie;
use crate::database::*;
use crate::model::Session;
use anyhow::Result;
use tower_cookies::Cookies;
use upholi_lib::ids::id;

// pub use files::*;
// pub use items::*;
// pub use shares::*;
// pub use user::*;

pub mod files;
pub mod items;
pub mod shares;
pub mod user;

/// Grant the current session access to given share ID. If no session exists, one is created.
async fn auth_share_for_session(existing_session: Option<Session>, cookies: Cookies, share_id: &str) -> Result<()> {
	let mut session = get_or_create_session(existing_session);

	if !session.shares.iter().any(|id| id == share_id) {
		session.shares.push(share_id.to_string());
		upsert_session(&session).await?
	}

	set_cookie(cookies, &session.id);
	Ok(())
}

/// Log given user ID into current session. If no session exists, one is created.
async fn auth_user_for_session(existing_session: Option<Session>, cookies: Cookies, user_id: &str) -> Result<()> {
	let mut session = get_or_create_session(existing_session);

	let some_user_id = Some(user_id.to_string());
	if session.user_id != some_user_id {
		session.user_id = some_user_id;
		upsert_session(&session).await?
	}

	set_cookie(cookies, &session.id);
	Ok(())
}

/// Get the session contained within the Option, otherwise create a new one.
fn get_or_create_session(existing_session: Option<Session>) -> Session {
	existing_session.unwrap_or_else(|| {
		let session_id = id();
		Session {
			id: session_id.clone(),
			user_id: None,
			shares: vec![],
		}
	})
}

/// Write the session cookie to the cookie container, ensuring it will be included in the response headers.
fn set_cookie(cookies: Cookies, session_id: &str) {
	let session_cookie = create_sesson_cookie(session_id.to_string());
	cookies.add(session_cookie);
}
