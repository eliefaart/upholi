use upholi_lib::http::request::EntityAuthorizationProof;

use session::Session;

pub mod album;
pub mod photo;
pub mod session;
pub mod share;
pub mod user;

pub trait AccessControl {
	fn can_view(&self, session: &Option<Session>, proof: Option<EntityAuthorizationProof>) -> bool;
	fn can_update(&self, session: &Option<Session>) -> bool;
	fn can_delete(&self, session: &Option<Session>) -> bool {
		// By default, delete rights are equal to update rights.
		self.can_update(session)
	}
}
