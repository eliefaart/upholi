use upholi_lib::http::EncryptedData;
use crate::aes256;
use crate::Result;

///
pub fn encrypt_slice(key: &[u8], data: &[u8]) -> Result<EncryptedData> {
	let nonce = aes256::generate_nonce();
	let encrypted = aes256::encrypt(&key, &nonce, data)?;

	Ok(EncryptedData {
		nonce: String::from_utf8(nonce)?,
		base64: base64::encode_config(&encrypted, base64::STANDARD)
	})
}

///
pub fn decrypt(key: &[u8], data: &EncryptedData) -> Result<Vec<u8>> {
	let nonce = data.nonce.as_bytes();
	let data = &base64::decode_config(&data.base64, base64::STANDARD)?;
	let decypted_bytes = aes256::decrypt(key, nonce, data)?;

	Ok(decypted_bytes)
}

pub fn decrypt_with_nonce(key: &[u8], nonce: &[u8], data: &[u8]) -> Result<Vec<u8>> {
	let decypted_bytes = aes256::decrypt(key, nonce, data)?;
	Ok(decypted_bytes)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn encrypt_data() {
		let key = b"e0ca4c29d5504e8daa8c52e873e66f71";
		let bytes = b"some kind of message";

		let encrypted_data = encrypt_slice(key, bytes).unwrap();

		assert!(encrypted_data.nonce.len() > 0);
		assert!(encrypted_data.base64.len() >= bytes.len());
	}

	#[test]
	fn encrypt_decrypt_data() {
		let key = b"e0ca4c29d5504e8daa8c52e873e66f71";
		let bytes = b"some kind of message";

		let encrypted_data = encrypt_slice(key, bytes).unwrap();
		let decrypted_data = decrypt(key, &encrypted_data).unwrap();

		assert_eq!(decrypted_data, bytes);
	}
}