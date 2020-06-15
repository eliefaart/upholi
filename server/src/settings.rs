use std::env::{var};

const ENV_VAR_DB_CONNECTION_STRING: &str = "HB_DB_CONNSTRING";
const ENV_VAR_DB_NAME: &str = "HB_DB_NAME";
const ENV_VAR_DIR_PHOTOS: &str = "HB_DIR_PHOTOS";
const ENV_VAR_OAUTH_CLIENT_ID: &str = "HB_OAUTH_CLIENTID";
const ENV_VAR_OAUTH_CLIENT_SECRET: &str = "HB_OAUTH_CLIENTSECRET";

/// Application settings
pub struct Settings {
	pub database: Database,
	pub photos: Photos,
	pub oauth: OAuth
}

/// Database settings
pub struct Database {
	pub connection_string: String,
	pub name: String
}

/// Photos settings
pub struct Photos {
	pub base_directory: String
}

/// OAuth setting of identity provider
pub struct OAuth {
	pub client_id: String,
	pub client_secret: String,
	pub auth_url: String,
	pub token_url: String,
	pub userinfo_url: String
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
			},
			oauth: OAuth{
				client_id: Self::get_env_var(ENV_VAR_OAUTH_CLIENT_ID),
				client_secret: Self::get_env_var(ENV_VAR_OAUTH_CLIENT_SECRET),
				auth_url: "https://github.com/login/oauth/authorize".to_string(),
				token_url: "https://github.com/login/oauth/access_token".to_string(),
				userinfo_url: "https://api.github.com/user".to_string(),
			}
		}
	}
	
	fn get_env_var(key: &str) -> String {
		let result = var(key);
		result.unwrap_or_else(|_| panic!("Environment variable with key '{}' missing", key))
	}
}