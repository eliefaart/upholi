use lazy_static::lazy_static;
use settings::Settings;

mod constants;
mod error;
mod database;
mod images;
mod storage;
mod ids;
mod settings;
mod web;
mod entities;
mod passwords;
mod encryption;

lazy_static! {
	/// Global application settings
	#[derive(Debug)]
	pub static ref SETTINGS: Settings = Settings::new();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	web::run_server().await
}