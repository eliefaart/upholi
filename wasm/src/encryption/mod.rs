use upholi_lib::ids::id_with_length;

mod aes128;

pub struct EncryptionResult {
    pub nonce: String,
    pub bytes: Vec<u8>,
}

fn generate_string(length: usize) -> Vec<u8> {
    id_with_length(length).as_bytes().to_vec()
}

pub mod symmetric {
    use super::{aes128, EncryptionResult};
    use anyhow::Result;

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
        let encrypted = aes128::encrypt(key, nonce, data)?;

        Ok(EncryptionResult {
            nonce: String::from_utf8(nonce.into())?,
            bytes: encrypted,
        })
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
            let decrypted_data = decrypt_slice(key, &encrypted_data.nonce.as_bytes(), &encrypted_data.bytes).unwrap();

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
