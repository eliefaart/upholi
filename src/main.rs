// I only need to include those 'extern' in main file?
extern crate actix_web;
extern crate actix_rt;
extern crate serde;
extern crate mongodb;
extern crate http;

use std::time::{Instant};
use actix_web::{web, App, HttpServer};
use actix_service::Service;
use futures::future::FutureExt;

mod types;
mod route_handlers;
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
			.route("/", web::get().to(route_handlers::route_index))

			.route("/albums", web::get().to(route_handlers::route_get_albums))
			.route("/album", web::post().to(route_handlers::route_insert_album))
			.route("/album/{album_id}", web::put().to(route_handlers::route_update_album))
			.route("/album/{album_id}", web::delete().to(route_handlers::route_delete_album))

			.route("/photos", web::get().to(route_handlers::route_get_photos))
			.route("/photo", web::post().to(route_handlers::route_upload_photo))
			.route("/photo/{photo_id}", web::get().to(route_handlers::route_get_photo))
			.route("/photo/{photo_id}/download", web::get().to(route_handlers::route_download_photo))
			.route("/photo/{photo_id}", web::put().to(route_handlers::route_update_photo))
			.route("/photo/{photo_id}", web::delete().to(route_handlers::route_delete_photo))
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}