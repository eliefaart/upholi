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

fn get_albums() -> Vec<Album> {
	let mut albums: Vec<Album> = Vec::new();
	let album_titles = ["Boom", "Hello world"];

	for id in 0..album_titles.len() {
		let album_title = album_titles[id];

		albums.push(Album{ id: id as u32, title: album_title.to_string() });
	}

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
		}
	}
}