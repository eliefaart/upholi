use crate::error::Result;
use crate::constants::{PASSWORD_HASH_ITERATIONS, PASSWORD_HASH_LENGTH};
use pbkdf2::{Params, Pbkdf2, password_hash::{Ident, PasswordHash, PasswordHasher, PasswordVerifier, Salt}};

/// Hashes using  algorithm pbkdf2-sha512
/// Returns the full PHC hash string
pub fn hash_password(password: &str, salt: &str) -> Result<String> {
    match Salt::new(salt) {
		Ok(salt) => {
			let params = Params {
				rounds: PASSWORD_HASH_ITERATIONS,
				output_length: PASSWORD_HASH_LENGTH,
			};
			let algorithm = Ident::new("pbkdf2-sha512");

			match Pbkdf2.hash_password(&password.as_bytes(), Some(algorithm), None, params.into(), salt) {
				Ok(password_hash) => Ok(password_hash.to_string()),
				Err(error) => Err(Box::from(error.to_string()))
			}
		},
		Err(error) => Err(Box::from(error.to_string()))
	}
}

/// Verify password against a PHC hash string
pub fn verify_password_hash(password: &str, phc_hash: &str) -> bool {
	match PasswordHash::new(phc_hash) {
		Ok(hash) => {
			match Pbkdf2.verify_password(password.as_bytes(), &hash) {
				Ok(_) => true,
				Err(_) => false
			}
		},
		Err(_) => false // Invalid PHC hash string
	}
}