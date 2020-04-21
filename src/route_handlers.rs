use actix_web::{web, HttpRequest, Responder};
use serde::{Serialize};

#[derive(Serialize)]
struct Album {
	id: u32,
	title: String
}

pub async fn handle_greet(req: HttpRequest) -> impl Responder {
	let name = req.match_info().get("name").unwrap_or("World");
	format!("Hello {}!", &name)
}

pub async fn handle_get_albums() -> impl Responder {
	let albums = get_albums();
	web::Json(albums)
}

fn get_albums() -> [Album;2] {
	let albums = [
		Album{ id: 1, title: "Boom".to_string() },
		Album{ id: 3, title: "Hello world".to_string() }
	];

	albums
}

#[cfg(test)]
mod tests {
	// Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

	#[test]
	fn test_get_albums() {
		let albums = get_albums();
		for album in albums.iter() {
			let album_title_length = album.title.len();

			assert!(album_title_length >= 1, "Album title is empty");
			assert!(album.id > 0, "Album ID is 0");
		}
	}
}