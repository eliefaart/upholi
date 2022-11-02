use anyhow::Result;
use core::fmt::Write;
use sha2::{Digest, Sha256};

pub fn compute_sha256_hash(bytes: &[u8]) -> Result<String> {
	let mut hasher = Sha256::new();
	hasher.update(bytes);
	let hash = hasher.finalize();

	// Convert hash bytes to hex string
	let hash = hash.as_slice();
	let mut hash_hex = String::with_capacity(2 * hash.len());
	for byte in hash {
		write!(hash_hex, "{:02x}", byte)?;
	}

	Ok(hash_hex)
}
