use std::error::Error;
use std::fmt::{self, Display};

/// A short alias for Result<T, Box<dyn std::error::Error>>, allows writing Result<T> instead
pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

/// Errors related to registering new users
#[derive(Debug)]
pub enum RegisterError {
	UsernameEmpty,
	PasswordTooShort,
}

/// Errors related to user logging in
#[derive(Debug)]
pub enum LoginError {
	InvalidCredentials,
}

/// Errors related to entity operations, such as CRUD operations on photos/albums/etc.
#[derive(Debug)]
pub enum EntityError {
	NoAccess,
}

/// Errors related to uploading files
#[derive(Debug)]
pub enum UploadError {
	HeaderContentDispositionInvalid,
	UnsupportedMultipartName,
}

/// Errors related to uploading files
#[derive(Debug)]
pub enum DatabaseError {
	InvalidId,
}

/// Errors linked to specific HTTP status codes
#[derive(Debug)]
pub enum HttpError {
	Unauthorized,
	NotFound,
}

impl Error for RegisterError {}
impl Error for LoginError {}
impl Error for EntityError {}
impl Error for UploadError {}
impl Error for DatabaseError {}
impl Error for HttpError {}

impl Display for RegisterError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				RegisterError::UsernameEmpty => "Username cannot be empty".to_string(),
				RegisterError::PasswordTooShort => {
					format!("Password must be at least {} characters", crate::SETTINGS.users.password_min_length)
				}
			}
		};
		write!(f, "{}", message)
	}
}

impl Display for LoginError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				LoginError::InvalidCredentials => "Invalid credentials",
			}
		};
		write!(f, "{}", message)
	}
}

impl Display for EntityError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				EntityError::NoAccess => "Access to entity not allowed for current user", // Note: 'current user' could be anonymous
			}
		};
		write!(f, "{}", message)
	}
}

impl Display for UploadError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				UploadError::HeaderContentDispositionInvalid => "Invalid header Content-Disposition",
				UploadError::UnsupportedMultipartName => "Multipart contains a part with an unsupported name",
			}
		};
		write!(f, "{}", message)
	}
}

impl Display for DatabaseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				DatabaseError::InvalidId => "Invalid id",
			}
		};
		write!(f, "{}", message)
	}
}

impl Display for HttpError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				HttpError::NotFound => String::from("Not found"),
				HttpError::Unauthorized => String::from("Unauthorized"),
			}
		};

		write!(f, "{}", message)
	}
}
