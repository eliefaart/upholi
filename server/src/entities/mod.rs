use user::User;

pub mod user;

pub trait AccessControl {
    fn user_has_access(&self, user: Option<User>) -> bool;
}