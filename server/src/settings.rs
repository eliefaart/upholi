use std::env::{var};
use config::{Config, File};
use serde::{Deserialize};
use std::collections::{HashMap};
use crate::error::*;

const ENV_VAR_DATABASE_CONNECTIONSTRING: &str = "HB_DATABASE_CONNECTIONSTRING";
const ENV_VAR_DATABASE_NAME: &str = "HB_DATABASE_NAME";
const ENV_VAR_STORAGE_DIRECTORYPHOTOS: &str = "HB_STORAGE_DIRECTORYPHOTOS";
const ENV_VAR_OAUTH_CLIENTID: &str = "HB_OAUTH_CLIENTID";
const ENV_VAR_OAUTH_CLIENTSECRET: &str = "HB_OAUTH_CLIENTSECRET";
const ENV_VAR_OAUTH_AUTHURL: &str = "HB_OAUTH_AUTHURL";
const ENV_VAR_OAUTH_TOKENURL: &str = "HB_OAUTH_TOKENURL";
const ENV_VAR_OAUTH_USERINFOURL: &str = "HB_OAUTH_USERINFOURL";

/// Application settings
#[derive(Debug, Deserialize)]
pub struct Settings {
	pub database: Database,
	pub storage: Storage,
	pub oauth: OAuth
}

/// Database settings
#[derive(Debug, Deserialize)]
pub struct Database {
	pub connection_string: String,
	pub name: String
}

/// Photos settings
#[derive(Debug, Deserialize)]
pub struct Storage {
	pub directory_photos: String
}

/// OAuth setting of identity provider
#[derive(Debug, Deserialize)]
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
	/// 
	/// # Panics
	/// 
	/// Panics if anything went wrong
	pub fn new() -> Self {

		let mut config = Config::new();

		// Set defaults from file
		config.merge(File::with_name("config/default")).expect("Default config file not found");
		
		// TODO: How does this work with field names containing underscores?
		// I would prefer to use this method instead of the solution below,
		// but not sure how to get around underscores issue.
		// Setting 'rename' or 'alias' via serde's attributes doesn't work.
		// config.merge(Environment::with_prefix("HB"))
		// 	.expect("Failed to set settings from env variables");

		// Set/overwrite certain settings from environment variables
		let overwritable_settings: HashMap<&str, &str> = [
			("database.connection_string", ENV_VAR_DATABASE_CONNECTIONSTRING),
			("database.name", ENV_VAR_DATABASE_NAME),
			("storage.directory_photos", ENV_VAR_STORAGE_DIRECTORYPHOTOS),
			("oauth.client_id", ENV_VAR_OAUTH_CLIENTID),
			("oauth.client_secret", ENV_VAR_OAUTH_CLIENTSECRET),
			("oauth.auth_url", ENV_VAR_OAUTH_AUTHURL),
			("oauth.token_url", ENV_VAR_OAUTH_TOKENURL),
			("oauth.userinfo_url", ENV_VAR_OAUTH_USERINFOURL),
		].iter().cloned().collect();

		for setting in overwritable_settings {
			Self::set_from_env_var(&mut config, setting.0, setting.1)
				.unwrap_or_else(|_| panic!("Failed to set '{}' from env variable '{}'", setting.0, setting.1));
		}

		// Build
		config.try_into().expect("Error building configuration")
	}

	// Write the value of an env var to configuration if the env var exists
	// This overwrites any existing value in configuration at given path.
	fn set_from_env_var(config: &mut Config, path: &str, env_var: &str) -> Result<()> {
		if let Some(env_var_value) = Self::get_env_var(env_var) {
			config.set(path, env_var_value)?;
		};
		Ok(())
	}
	
	/// Get the value of an environment variable if it exists
	fn get_env_var(key: &str) -> Option<String> {
		var(key).ok()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn no_env_vars() {
		Settings::new();
	}

	#[test]
	fn set_env_vars() {
		// Set some env vars
		std::env::set_var(ENV_VAR_DATABASE_CONNECTIONSTRING, "DATABASE_CONNECTIONSTRING");
		std::env::set_var(ENV_VAR_DATABASE_NAME, "DATABASE_NAME");
		std::env::set_var(ENV_VAR_STORAGE_DIRECTORYPHOTOS, "STORAGE_DIRECTORYPHOTOS");
		std::env::set_var(ENV_VAR_OAUTH_CLIENTID, "OAUTH_CLIENTID");
		std::env::set_var(ENV_VAR_OAUTH_CLIENTSECRET, "OAUTH_CLIENTSECRET");
		std::env::set_var(ENV_VAR_OAUTH_AUTHURL, "OAUTH_AUTHURL");
		std::env::set_var(ENV_VAR_OAUTH_TOKENURL, "OAUTH_TOKENURL");
		std::env::set_var(ENV_VAR_OAUTH_USERINFOURL, "OAUTH_USERINFOURL");

		// Create settings
		let settings = Settings::new();

		// Check if config settings' values are same as set in env vars
		assert_eq!(settings.database.connection_string, "DATABASE_CONNECTIONSTRING");
		assert_eq!(settings.database.name, "DATABASE_NAME");
		assert_eq!(settings.storage.directory_photos, "STORAGE_DIRECTORYPHOTOS");
		assert_eq!(settings.oauth.client_id, "OAUTH_CLIENTID");
		assert_eq!(settings.oauth.client_secret, "OAUTH_CLIENTSECRET");
		assert_eq!(settings.oauth.auth_url, "OAUTH_AUTHURL");
		assert_eq!(settings.oauth.token_url, "OAUTH_TOKENURL");
		assert_eq!(settings.oauth.userinfo_url, "OAUTH_USERINFOURL");
	}
}