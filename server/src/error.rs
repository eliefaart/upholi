use std::error::Error;
use std::fmt::{self, Display};

/// A short alias for Result<T, Box<dyn std::error::Error>>, allows writing Result<T> instead
pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

/// Errors related to entity operations, such as CRUD operations on photos/albums/etc.
#[derive(Debug)]
pub enum EntityError {
	IdMissing,
	AlreadyExists,
	NoAccess
}

/// Errors related to uploading files
#[derive(Debug)]
pub enum UploadError {
	HeaderContentDispositionInvalid,
	UnsupportedMultipartName
}

/// Errors related to uploading files
#[derive(Debug)]
pub enum DatabaseError {
	ReadCursorFailed,
	InvalidId
}

impl Error for EntityError {}
impl Error for UploadError {}
impl Error for DatabaseError {}

impl Display for EntityError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				EntityError::IdMissing => "Id field missing or empty",
				EntityError::AlreadyExists => "Entity already exists",
				EntityError::NoAccess => "Access to entity not allowed for current user"	// Note: 'current user' could be anonymous
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
				UploadError::UnsupportedMultipartName => "Multipart contains a part with an unsupported name"
			}
		};
		write!(f, "{}", message)
	}
}

impl Display for DatabaseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				DatabaseError::ReadCursorFailed => "Error reading item from database cursor",
				DatabaseError::InvalidId => "Invalid id"
			}
		};
		write!(f, "{}", message)
	}
}