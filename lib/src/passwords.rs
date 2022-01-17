use crate::result::Result;
use pbkdf2::{
	password_hash::{Ident, PasswordHash, PasswordHasher, PasswordVerifier, Salt},
	Params, Pbkdf2,
};

pub const PASSWORD_HASH_ITERATIONS: u32 = 4096;
pub const PASSWORD_HASH_LENGTH: usize = 64;

/// Hashes using algorithm pbkdf2-sha512
/// Returns the full PHC hash string
pub fn hash_password(password: &str, salt: &str) -> Result<String> {
	hash_password_with_length(password, salt, PASSWORD_HASH_LENGTH)
}

/// Hashes using algorithm pbkdf2-sha512
/// Returns the full PHC hash string.
///
/// # Arguments
///
/// * `length` - max length of hash.
pub fn hash_password_with_length(password: &str, salt: &str, length: usize) -> Result<String> {
	match Salt::new(salt) {
		Ok(salt) => {
			let params = Params {
				rounds: PASSWORD_HASH_ITERATIONS,
				output_length: length,
			};
			let algorithm = Ident::new("pbkdf2-sha512");

			match Pbkdf2.hash_password_customized(password.as_bytes(), Some(algorithm), None, params, salt) {
				Ok(phc) => Ok(phc.to_string()),
				Err(error) => Err(Box::from(error.to_string())),
			}
		}
		Err(error) => Err(Box::from(error.to_string())),
	}
}

/// Verify password against a PHC hash string
pub fn verify_password_hash(password: &str, phc_string: &str) -> bool {
	match PasswordHash::new(phc_string) {
		Ok(phc) => Pbkdf2.verify_password(password.as_bytes(), &phc).is_ok(),
		Err(_) => false, // Invalid PHC hash string
	}
}

/// Get the hash string of a full PHC string
pub fn get_hash_from_phc(phc_string: &str) -> Result<Vec<u8>> {
	match PasswordHash::new(phc_string) {
		Ok(phc) => {
			let hash = phc.hash.ok_or("PHC string missing hash")?;
			let hash = hash.as_bytes().to_vec();
			Ok(hash)
		}
		Err(error) => Err(Box::from(error.to_string())),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hash_verify_password() {
		let password = "password";
		let salt = "salt";

		let phc = hash_password(password, salt).unwrap();
		let valid = verify_password_hash(password, &phc);

		assert_eq!(valid, true);
	}

	#[test]
	fn password_hash_length() {
		let password = "password";
		let salt = "salt";
		let hash_length = 16;

		let phc = hash_password_with_length(password, salt, hash_length).unwrap();
		let hash = get_hash_from_phc(&phc).unwrap();

		assert_eq!(hash.len(), hash_length);
	}
}
