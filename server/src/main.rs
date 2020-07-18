use lazy_static::lazy_static;
use settings::Settings;

mod constants;
mod error;
mod database;
mod images;
mod files;
mod photos;
mod albums;
mod ids;
mod settings;
mod exif;
mod session;
mod web;

lazy_static! {
	/// Global application settings
	#[derive(Debug)]
	pub static ref SETTINGS: Settings = Settings::new();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	println!("{:?}", SETTINGS.database);

	web::run_server().await
}