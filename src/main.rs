extern crate actix_web;
extern crate actix_rt;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
	let name = req.match_info().get("name").unwrap_or("World");
	format!("Hello {}!", &name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	println!("Hello world.");

	HttpServer::new(|| {
		App::new()
			.route("/", web::get().to(greet))
			.route("/{name}", web::get().to(greet))
	})
	.bind("127.0.0.1:8000")?
	.run()
	.await
}

#[cfg(test)]
mod tests {

	#[test]
	fn test() {
		assert_eq!(1, 1);
	}
}