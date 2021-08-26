use upholi_lib::EncryptedData;
use crate::Result;

pub mod aes256;

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

/// Encrypt bytes
pub fn encrypt_slice(key: &[u8], data: &[u8]) -> Result<EncryptionResult> {
	let nonce = aes256::generate_nonce();
	let encrypted = aes256::encrypt(&key, &nonce, data)?;

	Ok(EncryptionResult {
		nonce: String::from_utf8(nonce)?,
		bytes: encrypted
	})
}

/// Decrypt an EncryptedData instance
pub fn decrypt_data(key: &[u8], data: &EncryptionResult) -> Result<Vec<u8>> {
	let nonce = data.nonce.as_bytes();
	let decypted_bytes = aes256::decrypt(key, nonce, &data.bytes)?;

	Ok(decypted_bytes)
}

/// Decrypt an EncryptedData instance
pub fn decrypt_data_base64(key: &[u8], data: &EncryptedData) -> Result<Vec<u8>> {
	decrypt_data(key, &data.to_owned().into())
}

/// Decrypt bytes
pub fn decrypt_slice(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<Vec<u8>> {
	let decypted_bytes = aes256::decrypt(key, nonce, data)?;
	Ok(decypted_bytes)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn encrypt() {
		let key = b"e0ca4c29d5504e8daa8c52e873e66f71";
		let bytes = b"some kind of message";

		let encrypted_data = encrypt_slice(key, bytes).unwrap();

		assert!(encrypted_data.nonce.len() > 0);
		assert!(encrypted_data.bytes.len() >= bytes.len());
	}

	#[test]
	fn encrypt_decrypt() {
		let key = b"e0ca4c29d5504e8daa8c52e873e66f71";
		let bytes = b"some kind of message";

		let encrypted_data = encrypt_slice(key, bytes).unwrap();
		let decrypted_data = decrypt_data(key, &encrypted_data).unwrap();

		assert_eq!(decrypted_data, bytes);
	}
}