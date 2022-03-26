use serde::{Deserialize, Serialize};
use session::Session;
use upholi_lib::http::request::EntityAuthorizationProof;

pub mod album;
pub mod photo;
pub mod session;
pub mod share;
pub mod user;

pub trait AccessControl {
	// TODO: A borrowed Option<T> is weird I guess. Refactor to Option<&T>

	fn can_view(&self, session: &Option<Session>, proof: Option<EntityAuthorizationProof>) -> bool;
	fn can_update(&self, session: &Option<Session>) -> bool;
	fn can_delete(&self, session: &Option<Session>) -> bool {
		// By default, delete rights are equal to update rights.
		self.can_update(session)
	}
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserEntity<T> {
	pub id: String,
	pub user_id: String,
	//pub access_proof: Option<String>,
	pub entity: T,
}

/// Check if UserEntity is owned by user of given session
fn session_owns_entity<T>(entity: &UserEntity<T>, session_opt: &Option<Session>) -> bool {
	if let Some(session) = session_opt {
		if let Some(user_id) = &session.user_id {
			if &entity.user_id == user_id {
				return true;
			}
		}
	}

	false
}
