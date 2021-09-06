use upholi_lib::passwords::{get_hash_from_phc, hash_password_with_length};
use upholi_lib::result::Result;
use aes_gcm_siv::{Aes128GcmSiv, Key, Nonce};
use aes_gcm_siv::aead::{Aead, NewAead};

const KEY_LENGTH: usize = 16;
const NONCE_LENGTH: usize = 12;

pub fn generate_key() -> Vec<u8> {
	super::generate_string(KEY_LENGTH)	// aes_gcm_siv should be able to do this?
}

pub fn generate_nonce() -> Vec<u8> {
	super::generate_string(NONCE_LENGTH)
}

pub fn derive_key_from_string(input: &str, salt: &str) -> Result<Vec<u8>> {
	let phc = hash_password_with_length(input, salt, KEY_LENGTH)?;
	let hash = get_hash_from_phc(&phc)?;
	Ok(hash.as_bytes().to_vec())
}

pub fn encrypt(key: &[u8], nonce: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
	if nonce.len() != NONCE_LENGTH {
		Err(Box::from(format!("Nonce must be {} bytes", NONCE_LENGTH)))
	}
	else {
		let cipher = get_cipher(key)?;
		let nonce = Nonce::from_slice(nonce);

		match cipher.encrypt(nonce, bytes.as_ref()) {
			Ok(encrypted_bytes) => Ok(encrypted_bytes),
			Err(error) => {
				println!("{}", error);
				Err(Box::from("Error encrypting bytes"))
			}
		}
	}
}

pub fn decrypt(key: &[u8], nonce: &[u8], bytes: &[u8]) -> Result<Vec<u8>> {
	if nonce.len() != NONCE_LENGTH {
		Err(Box::from(format!("Nonce must be {} bytes", NONCE_LENGTH)))
	}
	else {
		let cipher = get_cipher(key)?;
		let nonce = Nonce::from_slice(nonce);

		match cipher.decrypt(nonce, bytes.as_ref()) {
			Ok(decrypted_bytes) => Ok(decrypted_bytes),
			Err(error) => {
				println!("{}", error);
				Err(Box::from("Error decypting bytes"))
			}
		}
	}
}

fn get_cipher(key: &[u8]) -> Result<Aes128GcmSiv> {
	if key.len() != KEY_LENGTH {
		Err(Box::from(format!("Encryption key must be {} bytes", KEY_LENGTH)))
	}
	else {
		let key = Key::from_slice(key);
		Ok(Aes128GcmSiv::new(key))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn generate_key_length() {
		let key = generate_key();
		assert_eq!(key.len(), KEY_LENGTH);
	}

	#[test]
	fn generate_nonce_length() {
		let nonce = generate_key();
		assert_eq!(nonce.len(), NONCE_LENGTH);
	}

	#[test]
	fn encrypt_decrypt() {
		let key = b"e0ca4c29d5504e8d";
		let nonce = b"452b4dd698de";
		let bytes = b"message";

		let encrypted = encrypt(key, nonce, bytes).unwrap();
		let decrypted = decrypt(key, nonce, &encrypted).unwrap();

		assert_eq!(bytes.to_vec().len(), decrypted.len());
		assert_eq!(bytes.to_vec(), decrypted);
	}
}