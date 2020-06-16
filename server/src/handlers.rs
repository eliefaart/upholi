use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_multipart::{Multipart};
use actix_http::cookie::Cookie;
use serde::{Deserialize};

use crate::types;
use crate::database;
use crate::database::DatabaseOperations;
use crate::photos;
use crate::albums;
use crate::oauth2;
use crate::http::*;

/// Data send to oauth callback handler by oauth provider
#[derive(Deserialize)]
pub struct OauthCallbackRequest {
	code: String
	//state: String
}

/// Get all albums
pub async fn route_get_albums(_session: Session) -> impl Responder {
	HttpResponse::Ok().json(database::album::get_all())
}

/// Get extended information of an album
pub async fn route_get_album(req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();
	let result = database::album::get(album_id);

	match result {
		Some(album) => {
			let mut ids: Vec<&str> = Vec::new();

			for id in album.photos.iter() {
				ids.push(&id[..]);
			}
			
			let response = types::ClientAlbum {
				title: Some(album.title),
				thumb_photo: {
					if let Some(thumb_photo_id) = album.thumb_photo_id { 
						let result = database::photo::get(&thumb_photo_id);
						match result {
							Some(thumb_photo) => Some(thumb_photo.to_client_photo()),
							None => None
						}
					} else { 
						None 
					}
				},
				photos: {
					let result = database::photo::get_many(&ids);
					if let Some(photos) = result {
						let mut result_photos = Vec::new();
						for photo in photos {
							result_photos.push(photo.to_client_photo());
						}
	
						Some(result_photos)
					} else {
						None
					}
				}
			};
			HttpResponse::Ok().json(response)
		},
		None => create_not_found_response()
	}
}

/// Create a new album
pub async fn route_create_album(album: web::Json<albums::Album>) -> impl Responder {
	// TODO: Create different struct for request input. Should only have title, maybe other, but definetly not ID.
	let album = albums::Album::create(&album.title);
	let result = database::album::insert(&album);

	match result {
		Ok(_) => create_created_response(&album.id),
		Err(error) => create_internal_server_error_response(Some(&error))
	}
}

/// Update an album
pub async fn route_update_album(req: HttpRequest, album: web::Json<types::UpdateAlbum>) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	// TODO: Verify if all photoIds & thumbPhotoId are valid.

	let result = database::album::update(&album_id, &album);
	match result {
		Some(_) => create_ok_response(),
		None => create_not_found_response()
	}
}

/// Delete an album
pub async fn route_delete_album(req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();
	let result = database::album::delete(&album_id);

	match result {
		Some(_) => create_ok_response(),
		None => create_not_found_response()
	}
}

/// Get all photos
pub async fn route_get_photos() -> impl Responder {
	web::Json(database::photo::get_all())
}

/// Delete a single photo
pub async fn route_delete_photo(req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	delete_photos(&[photo_id])
}

/// Delete multiple photos
pub async fn route_delete_photos(photo_ids: web::Json<Vec<String>>) -> impl Responder {
	let mut ids: Vec<&str> = Vec::new();
	for id in photo_ids.iter() {
		ids.push(&id);
	}

	delete_photos(&ids)
}

/// Get info about a photo
pub async fn route_get_photo(req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(photo_id);

	match result {
		Some(photo) => HttpResponse::Ok().json(photo),
		None => create_not_found_response()
	}
}

/// Get the thumbnail of a photo as file
pub async fn route_download_photo_thumbnail(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(&_photo_id);

	match result {
		Some(photo_info) => serve_photo(&photo_info.path_thumbnail, &photo_info.name),
		None => create_not_found_response()
	}
}

/// Get the preview (large thumbnail) of a photo as file
pub async fn route_download_photo_preview(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(&_photo_id);

	match result {
		Some(photo_info) => serve_photo(&photo_info.path_preview, &photo_info.name),
		None => create_not_found_response()
	}
}

/// Get the original of a photo as file
pub async fn route_download_photo_original(req: HttpRequest) -> impl Responder {
	let _photo_id = req.match_info().get("photo_id").unwrap();
	let result = database::photo::get(&_photo_id);

	match result {
	Some(photo_info) => serve_photo(&photo_info.path_original, &photo_info.name),
		None => create_not_found_response()
	}
}

/// Upload a photo
pub async fn route_upload_photo(payload: Multipart) -> impl Responder {
	let form_data = get_form_data(payload).await;

	let mut files_iter = form_data.iter().filter(|d| d.name == "file");
	let file_option = files_iter.next();
	let remaining_files = files_iter.count();

	if remaining_files > 0 {
		return create_bad_request_response("Request contains more than one file.");
	}

	match file_option {
		Some(file) => {
			let result = photos::Photo::create(&file.bytes);
			match result {
				Ok(photo) => {
					let result = database::photo::insert(&photo);
					match result {
						Ok(_) => create_created_response(&photo.id),
						Err(error) => create_bad_request_response(&error)
					}
				},
				Err(error) => create_bad_request_response(&error)
			}
		},
		None => create_bad_request_response("Request contains no file.")
	}
}

/// OAuth: start login flow with an identity provider
pub async fn oauth_start_login() -> impl Responder {
	let redirect_uri = oauth2::get_auth_url();

	match database::session::Session::create() {
		Ok(session) => {
			// Create a new cookie for session
			// TODO: Make this expire after some amount of time, not permanent
			let mut cookie = Cookie::new(SESSION_COOKIE_NAME, session.id);
			cookie.set_secure(true);
			cookie.set_http_only(true);
			cookie.set_path("/");
			cookie.make_permanent();

			HttpResponse::Found()
				.cookie(cookie)
				.header(http::header::LOCATION, redirect_uri)
				.finish()
		},
		Err(_) => create_internal_server_error_response(None)
	}
}

/// OAuth callback
pub async fn oauth_callback(req: HttpRequest, oauth_info: web::Query<OauthCallbackRequest>) -> impl Responder {
	match oauth2::get_access_token(&oauth_info.code) {
		Ok(access_token) => {
			match oauth2::get_user_info(&access_token).await {
				Ok(user_info) => {
					match get_session_cookie(req.headers()) {
						Some(session_cookie) => {
							let session_id = session_cookie.value();
							match database::session::Session::get(&session_id) {
								Some(mut session) => {
									match session.set_user(user_info.id) {
										Ok(_) => {
											// Redirect to home page
											HttpResponse::Found()
												.header(http::header::LOCATION, "/")
												.finish()
										},
										Err(error) => create_internal_server_error_response(Some(&format!("{}", error)))
									}
								},
								None => create_unauthorized_response()
							}
						},
						None => create_unauthorized_response()
					}
				},
				Err(error) => {
					create_internal_server_error_response(Some(&format!("{}", error)))
				}
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// OAuth get info of current user
pub async fn oauth_user_info(session: Session) -> impl Responder {
	HttpResponse::Ok().json(session)
}