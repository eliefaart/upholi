use actix_web::{web, HttpRequest, Responder};
use http::{StatusCode};
use serde::{Serialize, Deserialize};

use crate::database;

#[derive(Serialize)]
struct Album {
	id: u32,
	title: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	name: String,
	content_type: String,
	base64: String
}

pub async fn handle_greet(req: HttpRequest) -> impl Responder {
	let name = req.match_info().get("name").unwrap_or("World");
	format!("Hello {}!", &name)
}

pub async fn handle_get_albums() -> impl Responder {
	let albums = get_albums();
	web::Json(albums)
}

pub async fn handle_insert_album() -> impl Responder {
	web::HttpResponse::build(StatusCode::OK)
}

pub async fn handle_insert_photo(photo_json: web::Json<Photo>) -> impl Responder {
	let photo = database::Photo{
		id: 0,
		name: "help".to_string(), //photo_json.name,
		width: 1,
		height: 1,
		landscape: true,
		date_taken: 0,
		path_thumbnail: "path_thumbnail".to_string(),
		path_preview: "path_preview".to_string(),
		path_original: "path_original".to_string()
	};
	database::add_photo(photo);
	web::HttpResponse::build(StatusCode::OK)
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