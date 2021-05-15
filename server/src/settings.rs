use std::env::var;
use config::{Config, File};
use serde::Deserialize;
use std::collections::HashMap;
use crate::error::*;

const ENV_VAR_SERVER_ADDRESS: &str = "HB_SERVER_ADDRESS";
const ENV_VAR_DATABASE_CONNECTIONSTRING: &str = "HB_DATABASE_CONNECTIONSTRING";
const ENV_VAR_DATABASE_NAME: &str = "HB_DATABASE_NAME";
const ENV_VAR_STORAGE_DIRECTORYPHOTOS: &str = "HB_STORAGE_DIRECTORYPHOTOS";
const ENV_VAR_STORAGE_AZURESTORAGEACCOUNTNAME: &str = "HB_STORAGE_AZURESTORAGEACCOUNTNAME";
const ENV_VAR_STORAGE_AZURESTORAGEACCOUNTKEY: &str = "HB_STORAGE_AZURESTORAGEACCOUNTKEY";

const ENV_VAR_OAUTH_PREFIX: &str = "HB_OAUTH";
const ENV_VAR_OAUTH_POSTFIX_CLIENTID: &str = "CLIENTID";
const ENV_VAR_OAUTH_POSTFIX_CLIENTSECRET: &str = "CLIENTSECRET";
const ENV_VAR_OAUTH_POSTFIX_AUTHURL: &str = "AUTHURL";
const ENV_VAR_OAUTH_POSTFIX_TOKENURL: &str = "TOKENURL";
const ENV_VAR_OAUTH_POSTFIX_USERINFOURL: &str = "USERINFOURL";

#[derive(Debug, Deserialize)]
pub enum StorageProvider {
	Disk,
	Azure
}
/// Application settings
#[derive(Debug, Deserialize)]
pub struct Settings {
	pub server: Server,
	pub database: Database,
	pub storage: Storage,
	pub oauth_providers: Vec<OAuthProvider>
}

/// Web server settings
#[derive(Debug, Deserialize)]
pub struct Server {
	pub address: String,
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
	pub provider: StorageProvider,
	pub directory_photos: String,
	pub azure_storage_account_name: String,
	pub azure_storage_account_key: String
}

/// OAuth setting of identity provider
#[derive(Debug, Deserialize)]
pub struct OAuthProvider {
	pub provider_id: String,
	pub client_id: String,
	pub client_secret: String,
	pub auth_url: String,
	pub token_url: String,
	pub userinfo_url: String
}

impl Settings {
	/// Get the settings of an OAuth provider by its ID
	pub fn get_oauth_provider_settings(&self, provider_id: &str) -> Option<&OAuthProvider> {
		for provider in &self.oauth_providers {
			if provider.provider_id == provider_id {
				return Some(provider);
			}
		}
		None
	}
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
			("server.address", ENV_VAR_SERVER_ADDRESS),
			("database.connection_string", ENV_VAR_DATABASE_CONNECTIONSTRING),
			("database.name", ENV_VAR_DATABASE_NAME),
			("storage.directory_photos", ENV_VAR_STORAGE_DIRECTORYPHOTOS),
			("storage.azure_storage_account_name", ENV_VAR_STORAGE_AZURESTORAGEACCOUNTNAME),
			("storage.azure_storage_account_key", ENV_VAR_STORAGE_AZURESTORAGEACCOUNTKEY),
		].iter().cloned().collect();

		for setting in overwritable_settings {
			Self::set_from_env_var(&mut config, setting.0, setting.1)
				.unwrap_or_else(|_| panic!("Failed to set '{}' from env variable '{}'", setting.0, setting.1));
		}

		// Build
		let mut settings: Settings = config.try_into().expect("Error building configuration");

		// Set/overwrite settings within oauth providers from environment variables
		// Unline non-auto provider fields, this happens after building the config because ```settings.oauth_providers``` is an array.
		for oauth_provider in &mut settings.oauth_providers {
			Self::update_oauth_provider_from_env_vars(oauth_provider);
		}

		settings
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

	/// Update fields of given OauthProvider from environment variables.
	///
	/// Looks for env vars with name format: HB_OAUTH_<PROVIDER_ID>_<FIELD_KEY>
	/// Where PROVIDER_ID equals the value in OAuthProvider.provider_id (ignoring case)
	/// And FIELD_KEY is one of the available fields within OAuthProvider:
	/// - CLIENTID
	///	- CLIENTSECRET
	///	- AUTHURL
	///	- TOKENURL
	///	- USERINFOURL
	/// For example:
	///  - HB_OAUTH_GITHUB_CLIENTSECRET
	fn update_oauth_provider_from_env_vars(oauth_provider: &mut OAuthProvider) {
		let id = &oauth_provider.provider_id;
		let env_var_prefix = format!("{}_{}_", ENV_VAR_OAUTH_PREFIX, id.to_uppercase());

		for (key, value) in std::env::vars() {
			if key.starts_with(&env_var_prefix) {
				let field_key = key.replace(&env_var_prefix, "");
				match field_key.as_str() {
					ENV_VAR_OAUTH_POSTFIX_CLIENTID => oauth_provider.client_id = value,
					ENV_VAR_OAUTH_POSTFIX_CLIENTSECRET => oauth_provider.client_secret = value,
					ENV_VAR_OAUTH_POSTFIX_AUTHURL => oauth_provider.auth_url = value,
					ENV_VAR_OAUTH_POSTFIX_TOKENURL => oauth_provider.token_url = value,
					ENV_VAR_OAUTH_POSTFIX_USERINFOURL => oauth_provider.userinfo_url = value,
					_ => {}
				}
			}
		}
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
		std::env::set_var(ENV_VAR_DATABASE_NAME, "DATABASE_NAME");
		std::env::set_var(ENV_VAR_STORAGE_DIRECTORYPHOTOS, "STORAGE_DIRECTORYPHOTOS");

		// Create settings
		let settings = Settings::new();

		// Check if config settings' values are same as set in env vars
		assert_eq!(settings.server.address, "SERVER_ADDRESS");
		assert_eq!(settings.database.connection_string, "DATABASE_CONNECTIONSTRING");
		assert_eq!(settings.database.name, "DATABASE_NAME");
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

	#[test]
	fn oauth_provider_env_vars() {
		const VAL_INITIAL: &str = "initial";
		const VAL_CHANGED: &str = "changed";

		let mut oauth_provider = OAuthProvider{
			provider_id: VAL_INITIAL.to_string(),
			client_id: VAL_INITIAL.to_string(),
			client_secret: VAL_INITIAL.to_string(),
			auth_url: VAL_INITIAL.to_string(),
			token_url: VAL_INITIAL.to_string(),
			userinfo_url: VAL_INITIAL.to_string(),
		};

		// Set some env vars; oauth_provider should contain these values after function
		std::env::set_var(format!("{}_INITIAL_{}", ENV_VAR_OAUTH_PREFIX, ENV_VAR_OAUTH_POSTFIX_CLIENTID), VAL_CHANGED);
		std::env::set_var(format!("{}_INITIAL_{}", ENV_VAR_OAUTH_PREFIX, ENV_VAR_OAUTH_POSTFIX_CLIENTSECRET), VAL_CHANGED);
		std::env::set_var(format!("{}_INITIAL_{}", ENV_VAR_OAUTH_PREFIX, ENV_VAR_OAUTH_POSTFIX_AUTHURL), VAL_CHANGED);
		std::env::set_var(format!("{}_INITIAL_{}", ENV_VAR_OAUTH_PREFIX, ENV_VAR_OAUTH_POSTFIX_TOKENURL), VAL_CHANGED);
		std::env::set_var(format!("{}_INITIAL_{}", ENV_VAR_OAUTH_PREFIX, ENV_VAR_OAUTH_POSTFIX_USERINFOURL), VAL_CHANGED);

		Settings::update_oauth_provider_from_env_vars(&mut oauth_provider);

		assert_eq!(oauth_provider.client_id, VAL_CHANGED);
		assert_eq!(oauth_provider.client_secret, VAL_CHANGED);
		assert_eq!(oauth_provider.auth_url, VAL_CHANGED);
		assert_eq!(oauth_provider.token_url, VAL_CHANGED);
		assert_eq!(oauth_provider.userinfo_url, VAL_CHANGED);
	}

	#[test]
	fn oauth_provider_no_env_vars() {
		const VAL_INITIAL: &str = "test";

		let mut oauth_provider = OAuthProvider{
			provider_id: VAL_INITIAL.to_string(),
			client_id: VAL_INITIAL.to_string(),
			client_secret: VAL_INITIAL.to_string(),
			auth_url: VAL_INITIAL.to_string(),
			token_url: VAL_INITIAL.to_string(),
			userinfo_url: VAL_INITIAL.to_string(),
		};

		// Do not set any env vars; values should remain unchanged

		Settings::update_oauth_provider_from_env_vars(&mut oauth_provider);

		assert_eq!(oauth_provider.client_id, VAL_INITIAL);
		assert_eq!(oauth_provider.client_secret, VAL_INITIAL);
		assert_eq!(oauth_provider.auth_url, VAL_INITIAL);
		assert_eq!(oauth_provider.token_url, VAL_INITIAL);
		assert_eq!(oauth_provider.userinfo_url, VAL_INITIAL);
	}
}