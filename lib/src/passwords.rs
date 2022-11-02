use anyhow::{anyhow, Result};
use pbkdf2::{
	password_hash::{PasswordHash, PasswordHasher, PasswordVerifier},
	Pbkdf2,
};

/// Hashes password using algorithm pbkdf2-sha512
/// Returns the full PHC hash string.
pub fn hash_password(password: &str) -> Result<String> {
	let salt = pbkdf2::password_hash::SaltString::generate(&mut pbkdf2::password_hash::rand_core::OsRng);
	hash_password_with_salt(password, salt.as_str())
}

/// Hashes password using algorithm pbkdf2-sha512 and using given salt
/// Returns the full PHC hash string.
pub fn hash_password_with_salt(password: &str, salt: &str) -> Result<String> {
	let salt = pbkdf2::password_hash::SaltString::b64_encode(salt.as_bytes()).map_err(|error| anyhow!("{error:?}"))?;
	let phc = Pbkdf2
		.hash_password(password.as_bytes(), &salt)
		.map_err(|error| anyhow!("{error:?}"))?;
	Ok(phc.to_string())
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
	let phc = PasswordHash::new(phc_string).map_err(|error| anyhow!("{error:?}"))?;
	let hash = phc.hash.ok_or_else(|| anyhow!("PHC string missing hash"))?;
	let hash = hash.as_bytes().to_vec();
	Ok(hash)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hash_verify_password() {
		let password = "password";

		let phc = hash_password(password).unwrap();
		let valid = verify_password_hash(password, &phc);

		assert_eq!(valid, true);
	}

	#[test]
	fn hash_verify_password_custom_salt() {
		let password = "password";
		let salt = "custom-salt";

		let phc = hash_password_with_salt(password, salt).unwrap();
		let valid = verify_password_hash(password, &phc);

		assert_eq!(valid, true);
	}
}
