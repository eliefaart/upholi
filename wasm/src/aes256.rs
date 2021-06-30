use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce};
use aes_gcm_siv::aead::{Aead, NewAead};

pub fn encrypt(key: &[u8], nonce: &[u8], bytes: &[u8]) -> crate::Result<Vec<u8>> {
	if nonce.len() != 12 {
		Err(Box::from("Nonce must be 12 bytes"))
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

pub fn decrypt(key: &[u8], nonce: &[u8], bytes: &[u8]) -> crate::Result<Vec<u8>> {
	if nonce.len() != 12 {
		Err(Box::from("Nonce must be 12 bytes"))
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

fn get_cipher(key: &[u8]) -> crate::Result<Aes256GcmSiv, > {
	if key.len() != 32 {
		Err(Box::from("Encryption key must be 32 bytes"))
	}
	else {
		let key = Key::from_slice(key);
		Ok(Aes256GcmSiv::new(key))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn encrypt_decrypt() {
		let key = b"e0ca4c29d5504e8daa8c52e873e66f71";
		let nonce = b"452b4dd698de";
		let bytes = b"message";

		let encrypted = encrypt(key, nonce, bytes).unwrap();
		let decrypted = decrypt(key, nonce, &encrypted).unwrap();

		assert_eq!(bytes.to_vec().len(), decrypted.len());
		assert_eq!(bytes.to_vec(), decrypted);
	}
}