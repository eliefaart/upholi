pub use album::*;
use anyhow::Result;
pub use auth_status::*;
use base64::prelude::*;
pub use library::*;
pub use photo::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use share::*;
pub use upload_queue::*;

mod album;
mod auth_status;
mod library;
mod photo;
mod share;
mod upload_queue;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncryptedItem {
    pub base64: String,
    pub nonce: String,
}

impl EncryptedItem {
    pub fn from<T: Serialize>(key: &[u8], item: &T) -> Result<Self> {
        let bytes = bincode::serialize(item)?;
        let encrypt_result = crate::encryption::symmetric::encrypt_slice(key, &bytes)?;
        let base64 = BASE64_STANDARD.encode(encrypt_result.bytes);
        Ok(Self {
            base64,
            nonce: encrypt_result.nonce,
        })
    }

    pub fn decrypt<TDecrypted: DeserializeOwned>(&self, key: &[u8]) -> Result<TDecrypted> {
        let nonce = self.nonce.as_bytes();
        let bytes = BASE64_STANDARD.decode(&self.base64)?;
        let bytes = crate::encryption::symmetric::decrypt_slice(key, nonce, &bytes)?;
        Ok(bincode::deserialize(&bytes)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        encryption::symmetric::generate_key,
        models::{EncryptedItem, Library},
    };

    #[test]
    fn encrypt_decrypt_text_item_bytes() {
        let key = generate_key();
        let item = EncryptedItem::from(&key, &key).unwrap();
        let decrypted: Vec<u8> = item.decrypt(&key).unwrap();

        assert_eq!(key, decrypted);
    }

    #[test]
    fn encrypt_decrypt_text_item_instance() {
        let key = generate_key();
        let library = Library::default();
        let item = EncryptedItem::from(&key, &library).unwrap();
        let decrypted: Library = item.decrypt(&key).unwrap();

        assert_eq!(library.photos.len(), decrypted.photos.len());
        assert_eq!(library.albums.len(), decrypted.albums.len());
        assert_eq!(library.shares.len(), decrypted.shares.len());
    }
}
