// I only need to include those 'extern' in main file?
extern crate actix_web;
extern crate actix_rt;
extern crate serde;
extern crate mongodb;
extern crate http;
extern crate rand;

use std::time::{Instant};
use actix_web::{web, App, HttpServer};
use actix_service::Service;
use futures::future::FutureExt;

mod types;
mod routes;
mod database;
mod images;
mod files;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

	HttpServer::new(|| {
		App::new()
			.wrap_fn(|req, srv| {
				// This is a middleware function
				let now = Instant::now();
				println!("> {method} {path}?{query_string}", 
					method = req.method(), 
					path = req.path(), 
					query_string = req.query_string());

				srv.call(req).map(move |res| {
					let elapsed_ms = now.elapsed().as_millis();
					println!("< {}ms", elapsed_ms);
					res
				})
			})
			.route("/", web::get().to(routes::route_index))

			.route("/albums", web::get().to(routes::route_get_albums))
			.route("/album", web::post().to(routes::route_create_album))
			.route("/album/{album_id}", web::get().to(routes::route_get_album))
			.route("/album/{album_id}", web::put().to(routes::route_update_album))
			.route("/album/{album_id}", web::delete().to(routes::route_delete_album))

			.route("/photos", web::get().to(routes::route_get_photos))
			.route("/photo", web::post().to(routes::route_upload_photo))
			.route("/photo/{photo_id}", web::get().to(routes::route_get_photo))
			.route("/photo/{photo_id}/original", web::get().to(routes::route_download_photo_original))
			.route("/photo/{photo_id}/thumb", web::get().to(routes::route_download_photo_thumbnail))
			.route("/photo/{photo_id}/preview", web::get().to(routes::route_download_photo_preview))
			.route("/photo/{photo_id}", web::put().to(routes::route_update_photo))
			.route("/photo/{photo_id}", web::delete().to(routes::route_delete_photo))
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}