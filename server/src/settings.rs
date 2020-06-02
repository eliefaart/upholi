use std::env::{var};

const ENV_VAR_DB_CONNECTION_STRING: &str = "HB_DB_CONNSTRING";
const ENV_VAR_DB_NAME: &str = "HB_DB_NAME";
const ENV_VAR_DIR_PHOTOS: &str = "HB_DIR_PHOTOS";

/// Application settings
#[derive(Debug)]
pub struct Settings {
	pub database: Database,
	pub photos: Photos
}

/// Database settings
#[derive(Debug)]
pub struct Database {
	pub connection_string: String,
	pub name: String
}

/// Photos settings
#[derive(Debug)]
pub struct Photos {
	pub base_directory: String
}

impl Default for Settings {
    fn default() -> Self {
		Self::new()
	}
}

impl Settings {
	/// Get all application settings
	/// It just panics now if something went wrong (eg missing env var)
    pub fn new() -> Self {
		Self {
			database: Database{
				connection_string: Self::get_env_var(ENV_VAR_DB_CONNECTION_STRING),
				name: Self::get_env_var(ENV_VAR_DB_NAME)
			},
			photos: Photos{
				base_directory: Self::get_env_var(ENV_VAR_DIR_PHOTOS)
			}
		}
	}
	
	fn get_env_var(key: &str) -> String {
		let result = var(key);
		result.unwrap_or_else(|_| panic!("Environment variable with key '{}' missing", key))
	}
}