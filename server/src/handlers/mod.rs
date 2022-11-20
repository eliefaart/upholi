use crate::database::*;
use crate::model::Session;
use anyhow::Result;

pub mod files;
pub mod items;
pub mod shares;
pub mod user;

/// Grant the current session access to given share ID. If no session exists, one is created.
async fn auth_share_for_session(mut session: Session, share_id: &str) -> Result<()> {
	if !session.shares.iter().any(|id| id == share_id) {
		session.shares.push(share_id.to_string());
		upsert_session(&session).await?
	}

	Ok(())
}

/// Log given user ID into current session. If no session exists, one is created.
async fn auth_user_for_session(mut session: Session, user_id: &str) -> Result<()> {
	let some_user_id = Some(user_id.to_string());
	if session.user_id != some_user_id {
		session.user_id = some_user_id;
		upsert_session(&session).await?
	}

	Ok(())
}
