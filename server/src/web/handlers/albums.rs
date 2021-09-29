use crate::entities::session::Session;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use upholi_lib::http::request::CreateAlbum;

use crate::database::{DatabaseEntity, DatabaseUserEntity};
use crate::web::http::*;
use crate::entities::AccessControl;
use crate::entities::user::User;
use crate::entities::album::Album;

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
						HttpResponse::Ok().json(album)
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
	let mut album = Album::from(album.into_inner());
	album.user_id = user.id;

	match album.insert() {
		Ok(_) => create_created_response(&album.id),
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Update an album
pub async fn route_update_album(session: Session, req: HttpRequest, updated_album: web::Json<CreateAlbum>) -> impl Responder {
	let album_id = req.match_info().get("album_id").unwrap();
	let updated_album = updated_album.into_inner();

	match &session.user_id {
		Some(user_id) => {
			match Album::get_as_user(&album_id, user_id.to_string()) {
				Ok(album_opt) => {
					match album_opt {
						Some(mut album) => {
							if !album.can_update(&Some(session)) {
								return create_unauthorized_response();
							}

							album.data = updated_album.data;
							album.key = updated_album.key;
							album.key_hash = updated_album.key_hash;

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