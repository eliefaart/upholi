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

mod route_handlers;
mod database;
mod images;

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
			.route("/", web::get().to(route_handlers::handle_greet))

			.route("/albums", web::get().to(route_handlers::handle_get_albums))
			.route("/albums", web::put().to(route_handlers::handle_insert_album))

			.route("/photos", web::put().to(route_handlers::handle_insert_photo))
			.route("/test", web::post().to(route_handlers::test_upload_photo))

			.route("/{name}", web::get().to(route_handlers::handle_greet))
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}