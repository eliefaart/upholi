use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_multipart::{Multipart};
use actix_http::cookie::Cookie;
use serde::{Deserialize};

use crate::types;
use crate::session::{Session};
use crate::database::{DatabaseOperations, DatabaseUserOperations};
use crate::photos;
use crate::photos::Photo;
use crate::albums;
use crate::albums::Album;
use crate::oauth2;
use crate::http::*;

/// Data send to oauth callback handler by oauth provider
#[derive(Deserialize)]
pub struct OauthCallbackRequest {
	code: String,
	state: String
}

#[derive(Deserialize)]
pub struct CreateAlbumRequest {
	title: String
}

/// Get all albums
pub async fn route_get_albums(user: User) -> impl Responder {
	match Album::get_all_as_user(user.user_id) {
		Ok(albums) => HttpResponse::Ok().json(albums),
		Err(error) => {
			println!("{}", error);
			create_internal_server_error_response(Some(&error))
		}
	}
}

/// Get extended information of an album
pub async fn route_get_album(user: User, req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	match Album::get_as_user(album_id, user.user_id) {
		Ok(album_opt) => {
			match album_opt {
				Some(album) => {
					let mut ids: Vec<&str> = Vec::new();
		
					for id in album.photos.iter() {
						ids.push(&id[..]);
					}
		
					let response = types::ClientAlbum {
						title: Some(album.title),
						thumb_photo: {
							if let Some(thumb_photo_id) = album.thumb_photo_id {
								match Photo::get_as_user(&thumb_photo_id, user.user_id) {
									Ok(photo_opt) => {
										match photo_opt {
											Some(thumb_photo) => Some(thumb_photo.to_client_photo()),
											None => None
										}
									},
									Err(_) => None
								}
							} else {
								None
							}
						},
						photos: {
							match Photo::get_all_with_ids_as_user(&ids, user.user_id) {
								Ok(photos) => {
									let mut result_photos = Vec::new();
									for photo in photos {
										result_photos.push(photo.to_client_photo());
									}
		
									Some(result_photos)
								}
								Err(_) => None
							}
						}
					};
					HttpResponse::Ok().json(response)
				},
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Create a new album
pub async fn route_create_album(user: User, album: web::Json<CreateAlbumRequest>) -> impl Responder {
	if album.title.len() > crate::constants::ALBUM_TITLE_MAX_LENGTH {
		create_bad_request_response(&format!("Maximum length for album title is {}.", crate::constants::ALBUM_TITLE_MAX_LENGTH))
	} else {
		let album = albums::Album::new(user.user_id, &album.title);

		match album.insert() {
			Ok(_) => create_created_response(&album.id),
			Err(error) => create_internal_server_error_response(Some(&error))
		}
	}
}

/// Update an album
pub async fn route_update_album(user: User, req: HttpRequest, updated_album: web::Json<types::UpdateAlbum>) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	match Album::get_as_user(&album_id, user.user_id) {
		Ok(album_opt) => {
			match album_opt {
				Some(mut album) => {
					if album.user_id != user.user_id {
						return create_unauthorized_response();
					}
		
					// TODO: Verify if all photoIds & thumbPhotoId are valid.
		
					if updated_album.title.is_some() {
						album.title = updated_album.title.as_ref().unwrap().to_string();
					}
					if updated_album.photos.is_some() {
						album.photos = updated_album.photos.as_ref().unwrap().to_vec();
					}
					if updated_album.thumb_photo_id.is_some() {
						album.thumb_photo_id = Some(updated_album.thumb_photo_id.as_ref().unwrap().to_string());
					}
		
					match album.update() {
						Ok(_) => create_ok_response(),
						Err(error) => create_internal_server_error_response(Some(&error))
					}
				},
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Delete an album
pub async fn route_delete_album(user: User, req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	match Album::get_as_user(&album_id, user.user_id) {
		Ok(album_opt) => {
			match album_opt {
				Some(album) => {
					match album.delete() {
						Ok(_) => create_ok_response(),
						Err(error) => create_internal_server_error_response(Some(&error))
					}
				},
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Get all photos
pub async fn route_get_photos(user: User) -> impl Responder {
	match Photo::get_all_as_user(user.user_id) {
		Ok(photos) => HttpResponse::Ok().json(photos),
		Err(error) => {
			println!("{}", error);
			create_internal_server_error_response(Some(&error))
		}
	}
}

/// Delete a single photo
pub async fn route_delete_photo(user: User, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	delete_photos(user.user_id, &[photo_id])
}

/// Delete multiple photos
pub async fn route_delete_photos(user: User, photo_ids: web::Json<Vec<String>>) -> impl Responder {
	let mut ids: Vec<&str> = Vec::new();
	for id in photo_ids.iter() {
		ids.push(&id);
	}

	delete_photos(user.user_id, &ids)
}

/// Get info about a photo
pub async fn route_get_photo(user: User, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	match Photo::get_as_user(photo_id, user.user_id) {
		Ok(photo_opt) => {
			match photo_opt {
				Some(photo) => HttpResponse::Ok().json(photo),
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Get the thumbnail of a photo as file
pub async fn route_download_photo_thumbnail(user: User, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	download_photo(photo_id, user.user_id, |photo| &photo.path_thumbnail).await
}

/// Get the preview (large thumbnail) of a photo as file
pub async fn route_download_photo_preview(user: User, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();
	
	download_photo(photo_id, user.user_id, |photo| &photo.path_preview).await
}

/// Get the original of a photo as file
pub async fn route_download_photo_original(user: User, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();
	
	download_photo(photo_id, user.user_id, |photo| &photo.path_original).await
}

/// Upload a photo
pub async fn route_upload_photo(user: User, payload: Multipart) -> impl Responder {
	let form_data = get_form_data(payload).await;

	let mut files_iter = form_data.iter().filter(|d| d.name == "file");
	let file_option = files_iter.next();
	let remaining_files = files_iter.count();

	if remaining_files > 0 {
		return create_bad_request_response("Request contains more than one file.");
	}

	match file_option {
		Some(file) => {
			match photos::Photo::new(user.user_id, &file.bytes) {
				Ok(photo) => {
					match photo.insert() {
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
	let (redirect_uri, state, pkce_verifier) = oauth2::get_auth_url();

	let mut session = Session::new();
	match session.insert() {
		Ok(_) => {
			session.set_oauth_data(&state, &pkce_verifier);
			match session.update() {
				Ok(_) => {
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
				Err(error) => create_internal_server_error_response(Some(&error))
			}
		},
		Err(error) => create_internal_server_error_response(Some(&error))
	}
}

/// OAuth callback
pub async fn oauth_callback(mut session: Session, oauth_info: web::Query<OauthCallbackRequest>) -> impl Responder {
	match &session.oauth {
		Some(oauth_data) => {
			// Verify state value
			if oauth_data.state != oauth_info.state {
				println!("Invalid oauth state provided");
				return create_unauthorized_response();
			}

			// Verify code externally
			match oauth2::get_access_token(&oauth_info.code, &oauth_data.pkce_verifier) {
				Ok(access_token) => {
					match oauth2::get_user_info(&access_token).await {
						Ok(user_info) => {
							// Assign the user to the session, and clear oauth login data/tokens
							session.set_user(user_info.id);
							session.oauth = None;

							match session.update() {
								Ok(_) => {
									// Redirect to home page
									HttpResponse::Found()
										.header(http::header::LOCATION, "/")
										.finish()
								},
								Err(error) => create_internal_server_error_response(Some(&format!("Error: {}", error)))
							}
						},
						Err(error) => {
							create_internal_server_error_response(Some(&format!("Error: {}", error)))
						}
					}
				},
				Err(error) => {
					println!("{}", error);
					create_unauthorized_response()
				}
			}
		},
		None => create_unauthorized_response()
	}
}

/// OAuth get info of current user
pub async fn oauth_user_info(user: User) -> impl Responder {
	HttpResponse::Ok().json(user)
}