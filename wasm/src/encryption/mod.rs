use upholi_lib::EncryptedData;

mod aes128;

pub struct EncryptionResult {
	pub nonce: String,
	pub bytes: Vec<u8>
}

impl From<EncryptedData> for EncryptionResult {
	fn from(source: EncryptedData) -> Self {
		Self {
			nonce: source.nonce.clone(),
			bytes: base64::decode_config(&source.base64, base64::STANDARD).unwrap_or_default()
		}
	}
}

impl From<EncryptionResult> for EncryptedData {
	fn from(source: EncryptionResult) -> Self {
		Self {
			nonce: source.nonce.clone(),
			base64: base64::encode_config(&source.bytes, base64::STANDARD),
			format_version: 1
		}
	}
}

fn generate_string(bytes: usize) -> Vec<u8> {
	// TODO: proper random bytes generation
	 uuid::Uuid::new_v4().to_simple().to_string()[0..bytes].as_bytes().to_vec()
}

pub mod symmetric {
	use upholi_lib::{result::Result, EncryptedData};
	use super::{EncryptionResult, aes128};

	pub fn generate_key() -> Vec<u8> {
		aes128::generate_key()
	}

	pub fn generate_nonce() -> Vec<u8> {
		aes128::generate_nonce()
	}

	pub fn derive_key_from_string(input: &str, salt: &str) -> Result<Vec<u8>> {
		aes128::derive_key_from_string(input, salt)
	}

	/// Encrypt bytes
	pub fn encrypt_slice(key: &[u8], data: &[u8]) -> Result<EncryptionResult> {
		let nonce = generate_nonce();
		encrypt_slice_with_nonce(key, &nonce, data)
	}

	/// Encrypt bytes
	pub fn encrypt_slice_with_nonce(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<EncryptionResult> {
		let encrypted = aes128::encrypt(&key, &nonce, data)?;

		Ok(EncryptionResult {
			nonce: String::from_utf8(nonce.into())?,
			bytes: encrypted
		})
	}

	/// Decrypt an EncryptedData instance
	pub fn decrypt_data(key: &[u8], data: &EncryptionResult) -> Result<Vec<u8>> {
		let nonce = data.nonce.as_bytes();
		let decypted_bytes = aes128::decrypt(key, nonce, &data.bytes)?;

		Ok(decypted_bytes)
	}

	/// Decrypt an EncryptedData instance
	pub fn decrypt_data_base64(key: &[u8], data: &EncryptedData) -> Result<Vec<u8>> {
		decrypt_data(key, &data.to_owned().into())
	}

	/// Decrypt bytes
	pub fn decrypt_slice(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<Vec<u8>> {
		let decypted_bytes = aes128::decrypt(key, nonce, data)?;
		Ok(decypted_bytes)
	}

	#[cfg(test)]
	mod tests {
		use super::*;

		#[test]
		fn encrypt() {
			let bytes = b"some kind of message";
			let key = &generate_key();

			let encrypted_data = encrypt_slice(key, bytes).unwrap();

			assert!(encrypted_data.nonce.len() > 0);
			assert!(encrypted_data.bytes.len() >= bytes.len());
		}

		#[test]
		fn encrypt_decrypt() {
			let bytes = b"some kind of message";
			let key = &generate_key();

			let encrypted_data = encrypt_slice(key, bytes).unwrap();
			let decrypted_data = decrypt_data(key, &encrypted_data).unwrap();

			assert_eq!(decrypted_data, bytes);
		}
	}
}

pub mod assymetric {
	// Don't need this so far
}

#[cfg(test)]
mod tests {
	#[test]
	fn generate_string() {
		let length = 32;
		let key = super::generate_string(length);

		assert_eq!(key.len(), length);
	}
}