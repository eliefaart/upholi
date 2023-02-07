use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::sync::RwLock;
use wasm_bindgen::UnwrapThrowExt;
use web_sys::Storage;

use crate::{encryption, hashing};

const LOCAL_STORAGE_KEY_MASTER_KEY: &str = "master-key";
const LOCAL_STORAGE_KEY_SHARE_KEY_PREFIX: &str = "share-key";

static MASTER_KEY: Lazy<RwLock<Vec<u8>>> = Lazy::new(|| {
    let storage = get_local_storage();
    let stored_key = storage.get_item(LOCAL_STORAGE_KEY_MASTER_KEY).unwrap_throw();
    let key = match stored_key {
        Some(key) => base64::decode_config(key, base64::STANDARD).unwrap_throw(),
        None => vec![],
    };

    RwLock::new(key)
});

/// Get user's master encryption key
pub fn get_master_key() -> Vec<u8> {
    MASTER_KEY.read().unwrap_throw().clone()
}

/// Set user's master encryption key
pub fn set_master_key(key: &Vec<u8>) {
    let key_str = &base64::encode_config(key, base64::STANDARD);
    let mut master_key = MASTER_KEY.write().unwrap_throw();
    *master_key = key.clone();

    let storage = get_local_storage();
    storage.set_item(LOCAL_STORAGE_KEY_MASTER_KEY, key_str).unwrap_throw();
}

/// Get a share's encryption key
pub fn get_share_key(share_id: &str) -> Result<Option<Vec<u8>>> {
    let storage = get_local_storage();
    let storage_key = get_storage_key_for_share(share_id);
    match storage.get_item(&storage_key).unwrap_throw() {
        Some(share_key) => {
            let share_key = base64::decode_config(share_key, base64::STANDARD)?;
            Ok(Some(share_key))
        }
        None => Ok(None),
    }
}

/// Set a share's encryption key
pub fn set_share_key(share_id: &str, key: &[u8]) -> Result<()> {
    let storage = get_local_storage();
    let storage_key = get_storage_key_for_share(share_id);
    let key_str = &base64::encode_config(key, base64::STANDARD);
    storage
        .set_item(&storage_key, key_str)
        .map_err(|_| anyhow!("Error writing share key to storage"))?;
    Ok(())
}

/// Derive a symmetric encryption key from a user's credentials
pub fn get_key_from_user_credentials(username: &str, password: &str) -> Result<Vec<u8>> {
    if username.is_empty() {
        Err(anyhow!("Username is empty"))
    } else if password.is_empty() {
        Err(anyhow!("Password is empty"))
    } else {
        // The salt is based on username; hash username to ensure minimum length.
        let salt = &hashing::compute_sha256_hash(username.as_bytes())?[..20];
        let password_derived_key = encryption::symmetric::derive_key_from_string(password, salt)?;
        Ok(password_derived_key)
    }
}

/// Get an instance to access browser's local storage
fn get_local_storage() -> Storage {
    web_sys::window()
        .expect_throw("no window")
        .local_storage()
        .unwrap_throw()
        .unwrap_throw()
}

fn get_storage_key_for_share(share_id: &str) -> String {
    format!("{LOCAL_STORAGE_KEY_SHARE_KEY_PREFIX}-{share_id}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_key_from_user_credentials_consistency() {
        let username = "username";
        let password = "password";

        let key_base = get_key_from_user_credentials(username, password).unwrap();

        // Identical credentials should give same key
        let key = get_key_from_user_credentials(username, password).unwrap();
        assert_eq!(key_base, key);

        // Any change in credentials should give a different key
        let key = get_key_from_user_credentials(username, "other_password").unwrap();
        assert_ne!(key_base, key);
        let key = get_key_from_user_credentials("other_username", password).unwrap();
        assert_ne!(key_base, key);
    }

    #[test]
    fn get_key_from_user_credentials_bad_input() {
        assert!(get_key_from_user_credentials("username", "").is_err());
        assert!(get_key_from_user_credentials("", "password").is_err());
        assert!(get_key_from_user_credentials("", "").is_err());
    }
}
