use std::time::Instant;
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use actix_service::Service;
use futures::future::FutureExt;

mod handlers;
mod http;
mod oauth2;

/// Start and run the web server
pub async fn run_server() -> std::io::Result<()>{
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
					.route("/start", actix_web::web::get().to(handlers::oauth_start_login))
					.route("/user/info", actix_web::web::get().to(handlers::oauth_user_info))
					.route("/login", actix_web::web::get().to(handlers::oauth_callback))
					//TODO:
					// .route("/logout", actix_web::web::get().to(handlers::logout))
			)
			.service(
				// API routes
				actix_web::web::scope("/api")
					.route("/albums", actix_web::web::get().to(handlers::route_get_albums))
					.route("/album", actix_web::web::post().to(handlers::route_create_album))
					.route("/album/{album_id}", actix_web::web::get().to(handlers::route_get_album))
					.route("/album/{album_id}", actix_web::web::put().to(handlers::route_update_album))
					.route("/album/{album_id}", actix_web::web::delete().to(handlers::route_delete_album))
					.route("/photos", actix_web::web::get().to(handlers::route_get_photos))
					.route("/photos", actix_web::web::delete().to(handlers::route_delete_photos))
					.route("/photo", actix_web::web::post().to(handlers::route_upload_photo))
					.route("/photo/{photo_id}", actix_web::web::get().to(handlers::route_get_photo))
					.route("/photo/{photo_id}/original", actix_web::web::get().to(handlers::route_download_photo_original))
					.route("/photo/{photo_id}/thumb", actix_web::web::get().to(handlers::route_download_photo_thumbnail))
					.route("/photo/{photo_id}/preview", actix_web::web::get().to(handlers::route_download_photo_preview))
					.route("/photo/{photo_id}", actix_web::web::delete().to(handlers::route_delete_photo))
			)
	})
	.bind(address)
	.unwrap_or_else(|_| panic!(format!("Failed to bind to {}, perhaps the port is in use?", address)))
	.run()
	.await
}