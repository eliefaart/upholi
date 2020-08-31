use std::time::Instant;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_web::http::header::{HeaderName,HeaderValue};
use actix_service::Service;
use actix_http::cookie::Cookie;
use futures::future::FutureExt;

use crate::database::DatabaseEntity;
use crate::entities::session::Session;
use crate::web::http::SESSION_COOKIE_NAME;

mod handlers;
mod http;
mod oauth2;

/// Start and run the web server
pub async fn run_server() -> std::io::Result<()>{
	let address = &crate::SETTINGS.server.address;
	println!("Hello server, address: {}", address);

	HttpServer::new(|| {
		App::new()
			.wrap(
				// https://docs.rs/actix-cors/0.2.0-alpha.3/actix_cors/struct.Cors.html
				// Allow everything by not specifying any origin/methods/etc
				Cors::new().finish()
			)
			.wrap_fn(|req, srv| {
				// Print all requests and responses to std-out
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
				// OAuth related routes
				actix_web::web::scope("/oauth")
					.route("/start", actix_web::web::get().to(handlers::oauth2::oauth_start_login))
					.route("/user/info", actix_web::web::get().to(handlers::oauth2::oauth_user_info))
					.route("/login", actix_web::web::get().to(handlers::oauth2::oauth_callback))
					//TODO:
					// .route("/logout", actix_web::web::get().to(handlers::logout))
			)
			.service(
				// API routes
				actix_web::web::scope("/api")
					.route("/photos", actix_web::web::get().to(handlers::photos::route_get_photos))
					.route("/photos", actix_web::web::delete().to(handlers::photos::route_delete_photos))
					.route("/photo", actix_web::web::post().to(handlers::photos::route_upload_photo))
					.route("/photo/{photo_id}", actix_web::web::get().to(handlers::photos::route_get_photo))
					.route("/photo/{photo_id}/original", actix_web::web::get().to(handlers::photos::route_download_photo_original))
					.route("/photo/{photo_id}/thumb", actix_web::web::get().to(handlers::photos::route_download_photo_thumbnail))
					.route("/photo/{photo_id}/preview", actix_web::web::get().to(handlers::photos::route_download_photo_preview))
					.route("/photo/{photo_id}", actix_web::web::delete().to(handlers::photos::route_delete_photo))
					
					.route("/albums", actix_web::web::get().to(handlers::albums::route_get_albums))
					.route("/album", actix_web::web::post().to(handlers::albums::route_create_album))
					.route("/album/{album_id}", actix_web::web::get().to(handlers::albums::route_get_album))
					.route("/album/{album_id}", actix_web::web::put().to(handlers::albums::route_update_album))
					.route("/album/{album_id}", actix_web::web::delete().to(handlers::albums::route_delete_album))

					.route("/collections", actix_web::web::get().to(handlers::collections::get_collections))
					.route("/collection", actix_web::web::post().to(handlers::collections::create_collection))
					.route("/collection/{collection_id}", actix_web::web::get().to(handlers::collections::get_collection))
					.route("/collection/shared/{token}", actix_web::web::get().to(handlers::collections::get_collections_by_share_token))
					.route("/collection/{collection_id}", actix_web::web::put().to(handlers::collections::update_collection))
					.route("/collection/{collection_id}", actix_web::web::delete().to(handlers::collections::delete_collection))
					.route("/collection/{collection_id}/rotate-token", actix_web::web::post().to(handlers::collections::rotate_collection_share_token))
			)
	})
	.bind(address)
	.unwrap_or_else(|_| panic!(format!("Failed to bind to {}, perhaps the port is in use?", address)))
	.run()
	.await
}