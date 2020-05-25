// I only need to include those 'extern' in main file?
extern crate actix_web;
extern crate actix_rt;
extern crate serde;
extern crate mongodb;
extern crate http;
extern crate rand;

use std::time::{Instant};
use actix_cors::Cors;
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
			.route("/", web::get().to(routes::route_index))

			.route("/albums", web::get().to(routes::route_get_albums))
			.route("/album", web::post().to(routes::route_create_album))
			.route("/album/{album_id}", web::get().to(routes::route_get_album))
			.route("/album/{album_id}", web::put().to(routes::route_update_album))
			.route("/album/{album_id}", web::delete().to(routes::route_delete_album))

			.route("/photos", web::get().to(routes::route_get_photos))
			.route("/photos", web::delete().to(routes::route_delete_photos))
			.route("/photo", web::post().to(routes::route_upload_photo))
			.route("/photo/{photo_id}", web::get().to(routes::route_get_photo))
			.route("/photo/{photo_id}/original", web::get().to(routes::route_download_photo_original))
			.route("/photo/{photo_id}/thumb", web::get().to(routes::route_download_photo_thumbnail))
			.route("/photo/{photo_id}/preview", web::get().to(routes::route_download_photo_preview))
			.route("/photo/{photo_id}", web::delete().to(routes::route_delete_photo))

			/*
				Two new collections:
				- Collections: {id, photoId[], createdOn}
				- SharedItems: {id, enum type: album | collection, itemId}
			*/
			// .route("/s/a/{sharedAlbumId}", web::get().to(routes::get_shared_album))
			// .route("/s/c/{sharedCollectionId}", web::get().to(routes::get_shared_collection))
			// .route("/s/", web::get().to(routes::get_shared_items))
			// .route("/s/{sharedItemId", web::delete().to(routes::delete_shared_item))
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}