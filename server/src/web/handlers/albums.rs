use crate::entities::session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::database::{DatabaseEntity, DatabaseUserEntity};
use crate::web::http::*;
use crate::entities::AccessControl;
use crate::entities::user::User;
use crate::entities::album::Album;
use crate::web::handlers::requests::*;
use crate::web::handlers::responses::*;

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
pub async fn route_get_album(session: Option<Session>, req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	match Album::get(album_id) {
		Ok(album_opt) => {
			match album_opt {
				Some(album) => {
					if album.can_view(&session) {
						HttpResponse::Ok().json(ClientAlbum::from(album))
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
pub async fn route_create_album(user: User, album: web::Json<CreateAlbum>) -> impl Responder {
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
pub async fn route_update_album(session: Session, req: HttpRequest, updated_album: web::Json<UpdateAlbum>) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	match &session.user_id {
		Some(user_id) => {
			match Album::get_as_user(&album_id, user_id.to_string()) {
				Ok(album_opt) => {
					match album_opt {
						Some(mut album) => {
							if !album.can_update(&Some(session)) {
								return create_unauthorized_response();
							}

							// TODO: Verify if all photoIds & thumbPhotoId are valid.

							if let Some(title) = &updated_album.title {
								album.title = title.to_string();
							}
							if let Some(thumb_photo_id) = &updated_album.thumb_photo_id {
								album.thumb_photo_id = Some(thumb_photo_id.to_string());
							}
							if let Some(photos) = &updated_album.photos {
								album.photos = photos.to_vec();
							}
							if let Some(tags) = &updated_album.tags {
								album.tags = tags.to_vec();
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
		},
		None => create_unauthorized_response()
	}

}

/// Delete an album
pub async fn route_delete_album(session: Session, req: HttpRequest) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();

	match &session.user_id {
		Some(user_id) => {
			match Album::get_as_user(&album_id, user_id.to_string()) {
				Ok(album_opt) => {
					match album_opt {
						Some(album) => {
							if !album.can_delete(&Some(session)) {
								return create_unauthorized_response();
							}

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
		},
		None => create_unauthorized_response()
	}
}