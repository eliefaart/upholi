use lazy_static::lazy_static;
use settings::Settings;

mod error;
mod database;
mod storage;
mod settings;
mod web;
mod entities;

lazy_static! {
	/// Global application settings
	#[derive(Debug)]
	pub static ref SETTINGS: Settings = Settings::new();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	web::run_server().await
}