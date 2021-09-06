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
					.route("/photo", actix_web::web::post().to(handlers::photos::route_upload_photo))
					.route("/photo/{photo_id}", actix_web::web::get().to(handlers::photos::route_get_photo))
					.route("/photo/{photo_id}", actix_web::web::delete().to(handlers::photos::route_delete_photo))
					.route("/photo/{photo_id}/original", actix_web::web::get().to(handlers::photos::route_download_photo_original))
					.route("/photo/{photo_id}/thumbnail", actix_web::web::get().to(handlers::photos::route_download_photo_thumbnail))
					.route("/photo/{photo_id}/preview", actix_web::web::get().to(handlers::photos::route_download_photo_preview))

					.route("/albums", actix_web::web::get().to(handlers::albums::route_get_albums))
					.route("/album", actix_web::web::post().to(handlers::albums::route_create_album))
					.route("/album/{album_id}", actix_web::web::get().to(handlers::albums::route_get_album))
					.route("/album/{album_id}", actix_web::web::put().to(handlers::albums::route_update_album))
					.route("/album/{album_id}", actix_web::web::delete().to(handlers::albums::route_delete_album))

					.route("/collections", actix_web::web::get().to(handlers::collections::get_collections))
					.route("/collection", actix_web::web::post().to(handlers::collections::create_collection))
					.route("/collection/{collection_id}", actix_web::web::get().to(handlers::collections::get_collection))
					.route("/collection/shared/{token}", actix_web::web::get().to(handlers::collections::get_collections_by_share_token))
					.route("/collection/shared/{token}/authenticate", actix_web::web::post().to(handlers::collections::authenticate_to_collection))
					.route("/collection/{collection_id}", actix_web::web::put().to(handlers::collections::update_collection))
					.route("/collection/{collection_id}", actix_web::web::delete().to(handlers::collections::delete_collection))
					.route("/collection/{collection_id}/rotate-token", actix_web::web::post().to(handlers::collections::rotate_collection_share_token))
			)
	})
	.bind(address)
	.unwrap_or_else(|_| panic!("Failed to bind to address, perhaps the port is in use?"))
	.run()
	.await
}