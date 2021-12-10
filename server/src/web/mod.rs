use actix_service::Service;
use actix_web::web::{delete, get, head, post, put};
use actix_web::{App, HttpServer};
use handlers::{albums::*, photos::*, shares::*, users::*};
use std::time::Instant;

mod cookies;
mod handlers;
mod http;

/// Start and run the web server
pub async fn run_server() -> std::io::Result<()> {
	let address = &crate::SETTINGS.server.address;
	println!("Hello server, address: {}", address);

	HttpServer::new(|| {
		App::new()
			// .wrap(
			// 	// https://docs.rs/actix-cors/0.2.0-alpha.3/actix_cors/struct.Cors.html
			// 	// Allow everything by not specifying any origin/methods/etc
			// 	Cors::default()
			// 	//Cors::new().finish()
			// )
			.wrap_fn(|req, srv| {
				// Middleware that prints all requests and responses to std-out
				let now = Instant::now();
				let query_id = format!(
					"{method} {path}?{query_string}",
					method = req.method(),
					path = req.path(),
					query_string = req.query_string()
				);

				println!(">> {}", query_id);

				let request_future = srv.call(req);
				async move {
					let response = request_future.await?;
					let elapsed_ms = now.elapsed().as_millis();
					println!("<< {} {}ms", query_id, elapsed_ms);
					Ok(response)
				}
			})
			.service(
				// API routes
				actix_web::web::scope("/api")
					.route("/user/register", post().to(route_register_user))
					.route("/user/login", post().to(route_login_user))
					//.route("/user/logout", post().to(route_logout_user))
					.route("/user/info", get().to(route_user_info))
					.route("/photos", get().to(route_get_photos))
					.route("/photo", head().to(route_check_photo_exists))
					.route("/photo", post().to(route_upload_photo))
					.route("/photo/{photo_id}", get().to(route_get_photo))
					.route("/photo/{photo_id}", delete().to(route_delete_photo))
					.route("/photo/{photo_id}/original", get().to(route_download_photo_original))
					.route("/photo/{photo_id}/thumbnail", get().to(route_download_photo_thumbnail))
					.route("/photo/{photo_id}/preview", get().to(route_download_photo_preview))
					// ?
					.route("/photos/find", post().to(route_find_photos))
					.route("/albums", get().to(route_get_albums))
					.route("/album", post().to(route_create_album))
					.route("/album/{album_id}", get().to(route_get_album))
					.route("/album/{album_id}", put().to(route_update_album))
					.route("/album/{album_id}", delete().to(route_delete_album))
					.route("/shares", get().to(route_get_shares))
					.route("/share", post().to(route_create_share))
					.route("/share/{share_id}", get().to(route_get_share))
					.route("/share/{share_id}", put().to(route_update_share))
					.route("/share/{share_id}", delete().to(route_delete_share)),
			)
	})
	.bind(address)
	.unwrap_or_else(|_| panic!("Failed to bind to address, perhaps the port is in use?"))
	.run()
	.await
}
