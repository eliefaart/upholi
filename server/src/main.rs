use std::time::Instant;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use actix_service::Service;
use lazy_static::lazy_static;
use settings::Settings;
use futures::future::{ok, Either, FutureExt};

mod types;
mod http;
mod handlers;
mod database;
mod images;
mod files;
mod photos;
mod albums;
mod ids;
mod settings;
mod exif;
mod oauth2;

lazy_static! {
	/// Global application settings
	#[derive(Debug)]
	pub static ref SETTINGS: Settings = Settings::new();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

	let address = "0.0.0.0:8000";
	println!("Hello server, address: {}", address);
	
	HttpServer::new(|| {
		App::new()
			.wrap(
				// https://docs.rs/actix-cors/0.2.0-alpha.3/actix_cors/struct.Cors.html
				// Allow everything by not specifying any origin/methods/etc
				Cors::new().finish()
			)
			.wrap_fn(|req, srv| {
				// This is a middleware function
				let now = Instant::now();
				let query_id = format!("{method} {path}?{query_string}", 
					method = req.method(), 
					path = req.path(), 
					query_string = req.query_string());

				println!(">> {}", query_id);

				srv.call(req).map(move |res| {
					let elapsed_ms = now.elapsed().as_millis();
					println!("<< {} {}ms", query_id, elapsed_ms);
					res
				})
			})
			.service(
				web::scope("/oauth")
					.route("/start", web::get().to(handlers::oauth_start_login))
					.route("/user/info", web::get().to(handlers::oauth_user_info))
					.route("/login", web::get().to(handlers::oauth_callback))
			)
			.service(
				web::scope("/api")
					.wrap_fn(|req, srv| {
						// If session cookie exists, then continue
						// Otherwise, abort request and return HTTP 401 unauthorized
						match http::get_session_cookie(&req.headers()) {
							Some(_) => srv.call(req),
							None => {
								Either::Right(ok(req.into_response(
									http::create_unauthorized_response()
								)))
							}
						}
					})
					.route("/albums", web::get().to(handlers::route_get_albums))
					.route("/album", web::post().to(handlers::route_create_album))
					.route("/album/{album_id}", web::get().to(handlers::route_get_album))
					.route("/album/{album_id}", web::put().to(handlers::route_update_album))
					.route("/album/{album_id}", web::delete().to(handlers::route_delete_album))
					.route("/photos", web::get().to(handlers::route_get_photos))
					.route("/photos", web::delete().to(handlers::route_delete_photos))
					.route("/photo", web::post().to(handlers::route_upload_photo))
					.route("/photo/{photo_id}", web::get().to(handlers::route_get_photo))
					.route("/photo/{photo_id}/original", web::get().to(handlers::route_download_photo_original))
					.route("/photo/{photo_id}/thumb", web::get().to(handlers::route_download_photo_thumbnail))
					.route("/photo/{photo_id}/preview", web::get().to(handlers::route_download_photo_preview))
					.route("/photo/{photo_id}", web::delete().to(handlers::route_delete_photo))

					/*
						Two new collections:
						- Collections: {id, photoId[], createdOn}
						- SharedItems: {id, enum type: album | collection, itemId}
					*/
					// .route("/s/a/{sharedAlbumId}", web::get().to(handlers::get_shared_album))
					// .route("/s/c/{sharedCollectionId}", web::get().to(handlers::get_shared_collection))
					// .route("/s/", web::get().to(handlers::get_shared_items))
					// .route("/s/{sharedItemId", web::delete().to(handlers::delete_shared_item))
			)
	})
	.bind(address)
	.expect(&format!("Failed to bind to {}, perhaps the port is in use?", address))
	.run()
	.await
}