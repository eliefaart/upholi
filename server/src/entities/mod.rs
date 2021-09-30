use upholi_lib::http::request::EntityAuthorizationProof;

use crate::entities::session::Session;

pub mod user;
pub mod session;
pub mod photo;
pub mod album;
pub mod share;

pub trait AccessControl {
    fn can_view(&self, session: &Option<Session>, proof: Option<EntityAuthorizationProof>) -> bool;
    fn can_update(&self, session: &Option<Session>) -> bool;
    fn can_delete(&self, session: &Option<Session>) -> bool {
        // By default, delete rights are equal to update rights.
        self.can_update(session)
    }
}