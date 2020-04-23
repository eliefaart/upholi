// I only need to include those 'extern' in main file?
extern crate actix_web;
extern crate actix_rt;
extern crate serde;
extern crate mongodb;

use actix_web::{web, App, HttpServer};

mod route_handlers;
mod database;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

	// let db = database::get_database().unwrap();
	// database::insert_docs(db);

	HttpServer::new(|| {
		App::new()
			.route("/", web::get().to(route_handlers::handle_greet))
			.route("/albums", web::get().to(route_handlers::handle_get_albums))
			.route("/albums", web::put().to(route_handlers::handle_create_album))
			.route("/{name}", web::get().to(route_handlers::handle_greet))
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}