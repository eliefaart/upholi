use uuid::Uuid;

/// Generate a new unique ID
pub fn create_unique_id() -> String {
	Uuid::new_v4().to_simple().to_string()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn create_unique_ids() {
		let id = create_unique_id();
		assert!(!id.is_empty());

		let mut prev_id = String::new();
		for _ in 0..10000 {
			let id = create_unique_id();
			assert_ne!(id, prev_id);
			prev_id = id;
		}
	}
}