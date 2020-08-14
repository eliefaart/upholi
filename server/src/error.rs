use std::error::Error;
use std::fmt;
use std::fmt::{Display};

/// A short alias for Result<T, Box<dyn std::error::Error>>, allows writing Result<T> instead
pub type Result<T, E = Box<dyn Error>> = std::result::Result<T, E>;

/// Errors related to entity operations, such as CRUD operations on photos/albums/etc.
#[derive(Debug)]
pub enum EntityError {
	IdMissing,
	AlreadyExists,
	NoAccess
}

/// Errors related to operations on physical files.
#[derive(Debug)]
pub enum FileError {
	InvalidFileName,
	NotFound
}

/// Errors related to oauth2 authentication
#[derive(Debug)]
pub enum Oauth2Error {
	GetAccessTokenFailed
}

/// Errors related to uploading files
#[derive(Debug)]
pub enum UploadError {
	NoFile,
	MoreThanOneFile,
	HeaderContentDispositionMissing,
	HeaderContentDispositionInvalid
}

/// Errors related to uploading files
#[derive(Debug)]
pub enum DatabaseError {
	ReadCursorFailed,
	InvalidId
}

impl Error for EntityError {}
impl Error for FileError {}
impl Error for Oauth2Error {}
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

impl Display for FileError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				FileError::InvalidFileName => "Invalid file name",
				FileError::NotFound => "File does not exist on disk"
			}
		};
		write!(f, "{}", message)
	}
}

impl Display for Oauth2Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				Oauth2Error::GetAccessTokenFailed => "Failed to get access token for auth code"
			}
		};
		write!(f, "{}", message)
	}
}

impl Display for UploadError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let message = {
			match self {
				UploadError::NoFile => "Request does not contain a file",
				UploadError::MoreThanOneFile => "Request contains more than one file",
				UploadError::HeaderContentDispositionMissing => "Missing header Content-Disposition",
				UploadError::HeaderContentDispositionInvalid => "Invalid header Content-Disposition"
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