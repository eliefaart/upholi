use crate::Result;
use config::{Config, File};
use serde::Deserialize;
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
        Self::try_new().unwrap()
    }

    fn try_new() -> Result<Self> {
        let builder = Config::builder()
            .add_source(File::with_name("config/default"))
            .set_override_option("server.address", var(ENV_VAR_SERVER_ADDRESS).ok())?
            .set_override_option("server.wwwroot_path", var(ENV_VAR_SERVER_WWWROOT_PATH).ok())?
            .set_override_option(
                "database.connection_string",
                var(ENV_VAR_DATABASE_CONNECTIONSTRING).ok(),
            )?
            .set_override_option("storage.provider", var(ENV_VAR_STORAGE_PROVIDER).ok())?
            .set_override_option("storage.directory_photos", var(ENV_VAR_STORAGE_DIRECTORYPHOTOS).ok())?
            .set_override_option(
                "storage.azure_storage_account_name",
                var(ENV_VAR_STORAGE_AZURESTORAGEACCOUNTNAME).ok(),
            )?
            .set_override_option(
                "storage.azure_storage_account_key",
                var(ENV_VAR_STORAGE_AZURESTORAGEACCOUNTKEY).ok(),
            )?;

        Ok(builder.build()?.try_deserialize::<Self>()?)
    }
}
