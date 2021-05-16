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

lazy_static! {
	/// Global application settings
	#[derive(Debug)]
	pub static ref SETTINGS: Settings = Settings::new();
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

	storage::test().await;

	web::run_server().await
}