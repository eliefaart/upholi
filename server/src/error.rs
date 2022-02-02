use actix_web::{HttpResponse, ResponseError};
use http::StatusCode;
use std::error::Error;
use std::fmt::{self, Display};
use upholi_lib::http::response::ErrorResult;

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

#[derive(Debug)]
pub enum HttpError {
	// /// HTTP 400
	// BadRequest(Box<dyn Error>),
	/// HTTP 401
	Unauthorized,
	/// HTTP 404
	NotFound,
	// /// HTTP 500, from a simple message
	// InternalServerErrorSimple(String),
	// /// HTTP 500, from any Error type
	// InternalServerError(Box<dyn Error>),
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
				// HttpError::InternalServerErrorSimple(message) => message.to_string(),
				// HttpError::InternalServerError(error) => format!("{:?}", error),
				_ => String::from("TODO"),
			}
		};

		write!(f, "{}", message)
	}
}

// impl ResponseError for HttpError {
// 	fn status_code(&self) -> StatusCode {
// 		println!("status_code");
// 		match self {
// 			HttpError::BadRequest(_) => StatusCode::BAD_REQUEST,
// 			HttpError::NotFound => StatusCode::NOT_FOUND,
// 			HttpError::Unauthorized => StatusCode::UNAUTHORIZED,
// 			_ => StatusCode::INTERNAL_SERVER_ERROR,
// 		}
// 	}

// 	fn error_response(&self) -> HttpResponse {
// 		println!("error_response");
// 		match self {
// 			HttpError::BadRequest(error) => HttpResponse::BadRequest().json(ErrorResult {
// 				message: format!("{:?}", error),
// 			}),
// 			HttpError::InternalServerErrorSimple(message) => HttpResponse::InternalServerError().json(ErrorResult {
// 				message: message.to_string(),
// 			}),
// 			HttpError::InternalServerError(error) => HttpResponse::InternalServerError().json(ErrorResult {
// 				message: format!("{:?}", error),
// 			}),
// 			HttpError::NotFound => HttpResponse::NotFound().finish(),
// 			HttpError::Unauthorized => HttpResponse::Unauthorized().finish(),
// 		}
// 	}
// }
