use actix_web::{web, HttpRequest, HttpResponse, Responder};
use actix_multipart::Multipart;
use actix_http::cookie::Cookie;

use crate::error::*;

use crate::database;
use crate::database::{Database, DatabaseExt, DatabaseEntity, DatabaseUserEntity};
use crate::files;
use crate::web::oauth2;
use crate::web::http::*;
use crate::entities::AccessControl;
use crate::entities::user::User;
use crate::entities::session::Session;
use crate::entities::photo::Photo;
use crate::entities::album::Album;

mod requests {
	use serde::Deserialize;

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct OauthCallback {
		pub code: String,
		pub state: String
	}

	#[derive(Deserialize)]
	#[serde(rename_all = "camelCase")]
	pub struct CreateAlbum {
		pub title: String
	}

	#[derive(Deserialize, Debug)]
	#[serde(rename_all = "camelCase")]
	pub struct UpdateAlbum {
		pub title: Option<String>,
		pub public: Option<bool>,
		pub thumb_photo_id: Option<String>,
		pub photos: Option<Vec<String>>
	}
}

mod responses {
	use serde::Serialize;
	use crate::entities::photo::Photo;
	use crate::entities::album::Album;
	use crate::database::{DatabaseEntity, DatabaseEntityBatch};
 
	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct PhotoSmall {
		id: String,
		width: u16,
		height: u16
	}

	#[derive(Serialize)]
	#[serde(rename_all = "camelCase")]
	pub struct ClientAlbum {
		pub title: String,
		pub public: bool,
		pub thumb_photo: Option<PhotoSmall>,
		pub photos: Vec<PhotoSmall>
	}
	
	impl From<Photo> for PhotoSmall {
		fn from(photo: Photo) -> Self {
			Self {
				id: photo.id,
				width: photo.width as u16,
				height: photo.height as u16
			}
		}
	}
	
	impl From<Album> for ClientAlbum {
		fn from(album: Album) -> Self {
			let mut ids: Vec<&str> = Vec::new();
			
			for id in album.photos.iter() {
				ids.push(&id[..]);
			}

			Self {
				title: album.title,
				public: album.public,
				thumb_photo: {
					if let Some(thumb_photo_id) = album.thumb_photo_id {
						match Photo::get(&thumb_photo_id) {
							Ok(thumb_photo_opt) => {
								match thumb_photo_opt {
									Some(thumb_photo) => Some(PhotoSmall::from(thumb_photo)),
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
					match Photo::get_with_ids(&ids) {
						Ok(photos) => {
							let mut result_photos = Vec::new();
							for photo in photos {
								result_photos.push(PhotoSmall::from(photo));
							}

							result_photos
						}
						Err(_) => vec!{}
					}
				}
			}
		}
	}
}

/// Get all albums
pub async fn route_get_albums(user: User) -> impl Responder {
	match Album::get_all_as_user(user.id) {
		Ok(albums) => HttpResponse::Ok().json(albums),
		Err(error) => {
			println!("{}", error);
			create_internal_server_error_response(Some(error))
		}
	}
}

/// Get extended information of an album
pub async fn route_get_album(user: Option<User>, req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	match Album::get(album_id) {
		Ok(album_opt) => {
			match album_opt {
				Some(album) => {
					if album.user_has_access(user) {
						HttpResponse::Ok().json(responses::ClientAlbum::from(album))
					} 
					else {
						create_unauthorized_response()
					}
				},
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Create a new album
pub async fn route_create_album(user: User, album: web::Json<requests::CreateAlbum>) -> impl Responder {
	if album.title.len() > crate::constants::ALBUM_TITLE_MAX_LENGTH {
		create_bad_request_response(Box::from(format!("Maximum length for album title is {}.", crate::constants::ALBUM_TITLE_MAX_LENGTH)))
	} else {
		let album = Album::new(user.id, &album.title);

		match album.insert() {
			Ok(_) => create_created_response(&album.id),
			Err(error) => create_internal_server_error_response(Some(error))
		}
	}
}

/// Update an album
pub async fn route_update_album(user: User, req: HttpRequest, updated_album: web::Json<requests::UpdateAlbum>) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	match Album::get_as_user(&album_id, user.id.to_string()) {
		Ok(album_opt) => {
			match album_opt {
				Some(mut album) => {
					if album.user_id != user.id {
						return create_unauthorized_response();
					}
		
					// TODO: Verify if all photoIds & thumbPhotoId are valid.
		
					if let Some(title) = &updated_album.title {
						album.title = title.to_string();
					}
					if let Some(public) = updated_album.public {
						album.public = public;
					}
					if let Some(photos) = &updated_album.photos {
						album.photos = photos.to_vec();
					}
					if let Some(thumb_photo_id) = &updated_album.thumb_photo_id {
						album.thumb_photo_id = Some(thumb_photo_id.to_string());
					}
		
					match album.update() {
						Ok(_) => create_ok_response(),
						Err(error) => create_internal_server_error_response(Some(error))
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

	match Album::get_as_user(&album_id, user.id) {
		Ok(album_opt) => {
			match album_opt {
				Some(album) => {
					match album.delete() {
						Ok(_) => create_ok_response(),
						Err(error) => create_internal_server_error_response(Some(error))
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
	match Photo::get_all_as_user(user.id) {
		Ok(photos) => {
			let photos_small: Vec<responses::PhotoSmall> = photos.into_iter()
				.map(responses::PhotoSmall::from)
				.collect();
			HttpResponse::Ok().json(photos_small)
		},
		Err(error) => {
			println!("{}", error);
			create_internal_server_error_response(Some(error))
		}
	}
}

/// Delete a single photo
pub async fn route_delete_photo(user: User, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	delete_photos(user.id, &[photo_id])
}

/// Delete multiple photos
pub async fn route_delete_photos(user: User, photo_ids: web::Json<Vec<String>>) -> impl Responder {
	let mut ids: Vec<&str> = Vec::new();
	for id in photo_ids.iter() {
		ids.push(&id);
	}

	delete_photos(user.id, &ids)
}

/// Get info about a photo
pub async fn route_get_photo(user: Option<User>, req: HttpRequest) -> impl Responder {
	let photo_id = req.match_info().get("photo_id").unwrap();

	match Photo::get(photo_id) {
		Ok(photo_opt) => {
			match photo_opt {
				Some(photo) => {
					if photo.user_has_access(user) {
						HttpResponse::Ok().json(photo)
					}
					else {
						create_unauthorized_response()
					}
				},
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Get the thumbnail of a photo as file
pub async fn route_download_photo_thumbnail(user: Option<User>, req: HttpRequest) -> impl Responder {
	match req.match_info().get("photo_id") {
		Some(photo_id) => create_response_for_photo(photo_id, user, false, |photo| &photo.path_thumbnail),
		None => create_not_found_response()
	}
}

/// Get the preview (large thumbnail) of a photo as file
pub async fn route_download_photo_preview(user: Option<User>, req: HttpRequest) -> impl Responder {
	match req.match_info().get("photo_id") {
		Some(photo_id) => create_response_for_photo(photo_id, user, false, |photo| &photo.path_preview),
		None => create_not_found_response()
	}
}

/// Get the original of a photo as file
pub async fn route_download_photo_original(user: Option<User>, req: HttpRequest) -> impl Responder {
	match req.match_info().get("photo_id") {
		Some(photo_id) => create_response_for_photo(photo_id, user, true, |photo| &photo.path_original),
		None => create_not_found_response()
	}
}

/// Upload a photo
pub async fn route_upload_photo(user: User, payload: Multipart) -> impl Responder {
	match get_form_data(payload).await {
		Ok(form_data) => {
			let mut files_iter = form_data.iter().filter(|d| d.name == "file");
			let file_option = files_iter.next();
			let remaining_files = files_iter.count();

			if remaining_files > 0 {
				return create_bad_request_response(Box::from(UploadError::MoreThanOneFile));
			}

			match file_option {
				Some(file) => {
					match Photo::new(user.id, &file.bytes) {
						Ok(photo) => {
							match photo.insert() {
								Ok(_) => create_created_response(&photo.id),
								Err(error) => create_bad_request_response(error)
							}
						},
						Err(error) => create_bad_request_response(error)
					}
				},
				None => create_bad_request_response(Box::from(UploadError::NoFile))
			}
		},
		Err(error) => create_bad_request_response(error)
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
				Err(error) => create_internal_server_error_response(Some(error))
			}
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// OAuth callback
pub async fn oauth_callback(mut session: Session, oauth_info: web::Query<requests::OauthCallback>) -> impl Responder {
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
							session.set_user(&user_info.id);
							session.oauth = None;

							match session.update() {
								Ok(_) => {
									// Redirect to home page
									HttpResponse::Found()
										.header(http::header::LOCATION, "/")
										.finish()
								},
								Err(error) => create_internal_server_error_response(Some(error))
							}
						},
						Err(error) => {
							create_internal_server_error_response(Some(error))
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

/// Get the HTTP response that returns a photo from disk by its id.
/// Given user must have access to it.
fn create_response_for_photo(photo_id: &str, user: Option<User>, offer_as_download: bool, select_path: fn(&Photo) -> &str) -> actix_http::Response {
	match Photo::get(photo_id) {
		Ok(photo_opt) => {
			match photo_opt {
				Some(photo_info) => {
					if photo_info.user_has_access(user) {
						serve_photo(&select_path(&photo_info), &photo_info.name, offer_as_download)
					}
					else {
						create_unauthorized_response()		
					}
				},
				None => create_not_found_response()
			}
		},
		Err(_) => create_unauthorized_response()
	}
}

/// Create an HTTP response that offers photo file at given path as download
fn serve_photo(path: &str, file_name: &str, offer_as_download: bool) -> actix_http::Response {
	match crate::files::get_photo(path) {
		Ok(file_bytes_option) => {
			match file_bytes_option {
				Some(file_bytes) => {
					HttpResponse::Ok()
						.content_type("image/jpeg")
						.header(http::header::CONTENT_DISPOSITION, 
							if offer_as_download {
								format!("attachment; filename=\"{}\"", file_name) 
							} else {  
								"inline;".to_string()
							})
						.body(file_bytes)
				},
				None => create_internal_server_error_response(Some(Box::from(FileError::NotFound)))
			}
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Delete multiple photos from database and disk
pub fn delete_photos(user_id: String, ids: &[&str]) -> impl Responder {
	// Check if all ids to be deleted are owned by user_id
	for id in ids {
		match Photo::get(id) {
			Ok(photo_opt) => {
				if let Some(photo) = photo_opt {
					if photo.user_id != user_id {
						return create_unauthorized_response();
					}
				}
			},
			Err(error) => return create_internal_server_error_response(Some(error))
		}
	}

	// Remove references to these photos from albums
	if let Err(error) = database::get_database().remove_photos_from_all_albums(ids) {
		return create_internal_server_error_response(Some(error));
	}
	if let Err(error) = database::get_database().remove_thumbs_from_all_albums(ids) {
		return create_internal_server_error_response(Some(error));
	}

	// Delete physical files for photo
	for id in ids {
		let result = delete_photo_files(&id);
		match result {
			Ok(_) => {},
			Err(error) => return create_internal_server_error_response(Some(error))
		}
	}

	// Delete all photos from database
	match database::get_database().delete_many(database::COLLECTION_PHOTOS, ids) {
		Ok(_) => create_ok_response(),
		Err(_) => create_not_found_response()
	}
}

/// Deletes all physical files of a photo from file system
/// Original, thumbnail and preview images.
fn delete_photo_files(photo_id: &str) -> Result<()> {
	if let Some(photo) = Photo::get(&photo_id)? {
		files::delete_photo(&photo.path_original)?;
		files::delete_photo(&photo.path_preview)?;
		files::delete_photo(&photo.path_thumbnail)?;
	}
	Ok(())
}
