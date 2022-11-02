use config::{Config, File};
use serde::Deserialize;
use std::collections::HashMap;
use std::env::var;

const ENV_VAR_SERVER_ADDRESS: &str = "UPHOLI_SERVER_ADDRESS";
const ENV_VAR_SERVER_WWWROOT_PATH: &str = "UPHOLI_SERVER_WWWROOT_PATH";
const ENV_VAR_DATABASE_CONNECTIONSTRING: &str = "UPHOLI_DATABASE_CONNECTIONSTRING";
const ENV_VAR_STORAGE_PROVIDER: &str = "UPHOLI_STORAGE_PROVIDER";
const ENV_VAR_STORAGE_DIRECTORYPHOTOS: &str = "UPHOLI_STORAGE_DIRECTORYPHOTOS";
const ENV_VAR_STORAGE_AZURESTORAGEACCOUNTNAME: &str = "UPHOLI_STORAGE_AZURESTORAGEACCOUNTNAME";
const ENV_VAR_STORAGE_AZURESTORAGEACCOUNTKEY: &str = "UPHOLI_STORAGE_AZURESTORAGEACCOUNTKEY";

#[derive(Debug, Deserialize)]
pub enum StorageProvider {
	Disk,
	Azure,
}
/// Application settings
#[derive(Debug, Deserialize)]
pub struct Settings {
	pub server: Server,
	pub database: Database,
	pub storage: Storage,
}

/// Web server settings
#[derive(Debug, Deserialize)]
pub struct Server {
	pub address: String,
	pub wwwroot_path: String,
}

/// Database settings
#[derive(Debug, Deserialize)]
pub struct Database {
	pub connection_string: String,
}

/// Storage settings
#[derive(Debug, Deserialize)]
pub struct Storage {
	pub provider: StorageProvider,
	pub directory_photos: String,
	pub azure_storage_account_name: String,
	pub azure_storage_account_key: String,
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
		config
			.merge(File::with_name("config/default"))
			.expect("Default config file not found");

		// Set/overwrite certain settings from environment variables
		let overwritable_settings: HashMap<&str, &str> = [
			("server.address", ENV_VAR_SERVER_ADDRESS),
			("server.wwwroot_path", ENV_VAR_SERVER_WWWROOT_PATH),
			("database.connection_string", ENV_VAR_DATABASE_CONNECTIONSTRING),
			("storage.provider", ENV_VAR_STORAGE_PROVIDER),
			("storage.directory_photos", ENV_VAR_STORAGE_DIRECTORYPHOTOS),
			("storage.azure_storage_account_name", ENV_VAR_STORAGE_AZURESTORAGEACCOUNTNAME),
			("storage.azure_storage_account_key", ENV_VAR_STORAGE_AZURESTORAGEACCOUNTKEY),
		]
		.iter()
		.cloned()
		.collect();

		for setting in overwritable_settings {
			Self::set_from_env_var(&mut config, setting.0, setting.1)
				.unwrap_or_else(|_| panic!("Failed to set '{}' from env variable '{}'", setting.0, setting.1));
		}

		// Build
		let settings: Settings = config.try_into().expect("Error building configuration");
		settings
	}

	// Write the value of an env var to configuration if the env var exists
	// This overwrites any existing value in configuration at given path.
	fn set_from_env_var(config: &mut Config, path: &str, env_var: &str) -> anyhow::Result<()> {
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
		std::env::set_var(ENV_VAR_SERVER_ADDRESS, "SERVER_ADDRESS");
		std::env::set_var(ENV_VAR_DATABASE_CONNECTIONSTRING, "DATABASE_CONNECTIONSTRING");
		std::env::set_var(ENV_VAR_STORAGE_DIRECTORYPHOTOS, "STORAGE_DIRECTORYPHOTOS");

		// Create settings
		let settings = Settings::new();

		// Check if config settings' values are same as set in env vars
		assert_eq!(settings.server.address, "SERVER_ADDRESS");
		assert_eq!(settings.database.connection_string, "DATABASE_CONNECTIONSTRING");
		assert_eq!(settings.storage.directory_photos, "STORAGE_DIRECTORYPHOTOS");
	}

	#[test]
	fn get_env_var() {
		std::env::set_var("EXISTS", "HELLO WORLD");

		let exists = Settings::get_env_var("EXISTS");
		let not_exists = Settings::get_env_var("NOT_EXISTS");

		assert!(exists.is_some() && exists.unwrap() == "HELLO WORLD");
		assert_eq!(not_exists, None);
	}
}
