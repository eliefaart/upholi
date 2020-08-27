use user::User;

pub mod user;
pub mod session;
pub mod photo;
pub mod album;
pub mod exif;
pub mod collection;

pub trait AccessControl {
    fn user_has_access(&self, user: Option<User>) -> bool;
}