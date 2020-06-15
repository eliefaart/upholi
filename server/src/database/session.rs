use crate::ids;

pub struct Session {
	pub id: String,
	pub user_id: Option<u64>,

	/// Contains data related to an oauth login attempt
	/// Such as: state id, PKCE tokens. 
	/// The value of this field will be None if login has completed
	pub _oauth: Option<()>
}

impl Session {
	pub fn new() -> Self {
		Self {
			id: ids::create_unique_id(),
			user_id: None,
			_oauth: None
		}
	}

	pub fn get(session_id: &str) -> Option<Self> {
		Some(Self {
			id: session_id.to_string(),
			user_id: Some(99999u64),
			_oauth: None
		})
	}

	pub fn set_user(&mut self, user_id: u64) {
		self.user_id = Some(user_id);

		Self::update(self);
	}

	fn update(&self) {

	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn new() {
		let session = Session::new();

		assert!(session.id.len() != 0);
	}

	#[test]
	fn set_user() {
		const USER_ID: u64 = 999555u64;

		let mut session = Session::new();
		session.set_user(USER_ID);

		assert!(session.user_id.is_some());
		assert_eq!(session.user_id.unwrap(), USER_ID);
	}
}