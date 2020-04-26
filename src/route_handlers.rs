use actix_web::{web, HttpRequest, Responder};
use actix_multipart::Multipart;
use http::{StatusCode};
use serde::{Serialize, Deserialize};
use image::GenericImageView;
use futures::stream::Stream;

use crate::database;
use crate::images;

#[derive(Serialize)]
struct Album {
	id: u32,
	title: String
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
	name: String,
	//content_type: String,
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

pub async fn test_upload_photo(payload: Multipart/*, photo_json: web::Json<Photo>*/) -> impl Responder {
	
	let size = payload.size_hint();
	println!("{:?}", size);
	//println!("{}", photo_json.name);

	// iterate over multipart stream
    // while let Ok(Some(mut field)) = payload.try_next().await {
    //     let content_type = field.content_disposition().unwrap();
    //     let filename = content_type.get_filename().unwrap();
    //     let filepath = format!("./tmp/{}", filename);
    //     // File::create is blocking operation, use threadpool
    //     let mut f = web::block(|| std::fs::File::create(filepath))
    //         .await
    //         .unwrap();
    //     // Field in turn is stream of *Bytes* object
    //     while let Some(chunk) = field.next().await {
    //         let data = chunk.unwrap();
    //         // filesystem operations are blocking, we have to use threadpool
    //         f = web::block(move || f.write_all(&data).map(|_| f)).await?;
    //     }
    // }

	web::HttpResponse::build(StatusCode::OK)
}


pub async fn handle_insert_photo(photo_json: web::Json<Photo>) -> impl Responder {

	const DIMENSIONS_THUMB: u32 = 300;
	const DIMENSIONS_PREVIEW: u32 = 1250;

	// std::env::current_dir().unwrap().to_string_lossy()
	let photo_name = &photo_json.name;

	// Convert base64 string to image
	let photo_bytes = images::from_base64(&photo_json.base64);
	let image_result = image::load_from_memory(&photo_bytes[0..]);
	let image = image_result.unwrap();

	let image_path = format!("C:/Development/_TestData/hummingbird/photos/{}", photo_name);
	let mut thumbnail_path = format!("C:/Development/_TestData/hummingbird/photos/thumb_{}", photo_name);
	let mut preview_path = format!("C:/Development/_TestData/hummingbird/photos/preview_{}", photo_name);

	// Save original image to disk
	let save_result = image.save(&image_path);
	println!("{:?}", save_result);

	let photo_width = image.width();
	let photo_height = image.height();

	// Create thumbnail image
	if photo_width > DIMENSIONS_THUMB && photo_height > DIMENSIONS_THUMB {
		let image_thumbnail = image.thumbnail(DIMENSIONS_THUMB, DIMENSIONS_THUMB);
		let save_result = image_thumbnail.save(&thumbnail_path);
		println!("{:?}", save_result);
	} 
	else {
		thumbnail_path = String::from(&image_path);
	}

	// Create preview image
	if photo_width > DIMENSIONS_THUMB && photo_height > DIMENSIONS_THUMB {
		let image_preview = image.thumbnail(DIMENSIONS_PREVIEW, DIMENSIONS_PREVIEW);
		let save_result = image_preview.save(&preview_path);
		println!("{:?}", save_result);
	}
	else {
		preview_path = String::from(&image_path);
	}

	let photo = database::Photo{
		name: photo_name.to_string(),
		width: photo_width,
		height: photo_height,
		landscape: true,
		date_taken: 0,
		path_thumbnail: thumbnail_path,
		path_preview: preview_path,
		path_original: image_path
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