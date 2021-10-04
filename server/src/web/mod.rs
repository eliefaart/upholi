use std::time::Instant;

use actix_service::Service;
use actix_web::{App, HttpServer};

mod handlers;
mod http;
mod cookies;

/// Start and run the web server
pub async fn run_server() -> std::io::Result<()>{
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
				let query_id = format!("{method} {path}?{query_string}",
					method = req.method(),
					path = req.path(),
					query_string = req.query_string());

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

					.route("/user/register", actix_web::web::post().to(handlers::users::route_register_user))
					.route("/user/login", actix_web::web::post().to(handlers::users::route_login_user))
					//.route("/user/logout", actix_web::web::post().to(handlers::users::route_logout_user))
					.route("/user/info", actix_web::web::get().to(handlers::users::route_user_info))

					.route("/photos", actix_web::web::get().to(handlers::photos::route_get_photos))
					.route("/photo", actix_web::web::head().to(handlers::photos::route_check_photo_exists))
					.route("/photo", actix_web::web::post().to(handlers::photos::route_upload_photo))
					.route("/photo/{photo_id}", actix_web::web::get().to(handlers::photos::route_get_photo))
					.route("/photo/{photo_id}", actix_web::web::delete().to(handlers::photos::route_delete_photo))
					.route("/photo/{photo_id}/original", actix_web::web::get().to(handlers::photos::route_download_photo_original))
					.route("/photo/{photo_id}/thumbnail", actix_web::web::get().to(handlers::photos::route_download_photo_thumbnail))
					.route("/photo/{photo_id}/preview", actix_web::web::get().to(handlers::photos::route_download_photo_preview))

					// ?
					.route("/photos/find", actix_web::web::post().to(handlers::photos::route_find_photos))

					.route("/albums", actix_web::web::get().to(handlers::albums::route_get_albums))
					.route("/album", actix_web::web::post().to(handlers::albums::route_create_album))
					.route("/album/{album_id}", actix_web::web::get().to(handlers::albums::route_get_album))
					.route("/album/{album_id}", actix_web::web::put().to(handlers::albums::route_update_album))
					.route("/album/{album_id}", actix_web::web::delete().to(handlers::albums::route_delete_album))

					.route("/shares", actix_web::web::get().to(handlers::shares::route_get_shares))
					.route("/share", actix_web::web::post().to(handlers::shares::route_create_share))
					.route("/share/{share_id}", actix_web::web::get().to(handlers::shares::route_get_share))
					.route("/share/{share_id}", actix_web::web::put().to(handlers::shares::route_update_share))
					.route("/share/{share_id}", actix_web::web::delete().to(handlers::shares::route_delete_share))
			)
	})
	.bind(address)
	.unwrap_or_else(|_| panic!("Failed to bind to address, perhaps the port is in use?"))
	.run()
	.await
}