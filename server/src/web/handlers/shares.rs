use crate::database::entities::session::Session;
use crate::database::entities::share::Share;
use crate::database::entities::user::User;
use crate::database::entities::AccessControl;
use crate::database::{DatabaseEntity, DatabaseUserEntity};
use crate::web::http::*;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use upholi_lib::http::request::{FindSharesFilter, UpsertShare};

/// Get all shares
pub async fn route_get_shares(user: User, filters: web::Query<FindSharesFilter>) -> impl Responder {
	let filters = filters.into_inner();
	match Share::find_shares(&user.id, filters).await {
		Ok(shares) => HttpResponse::Ok().json(shares),
		Err(error) => {
			println!("{}", error);
			create_internal_server_error_response(Some(error))
		}
	}
}

/// Get extended information of an share
pub async fn route_get_share(session: Option<Session>, req: HttpRequest) -> impl Responder {
	let share_id = req.match_info().get("share_id").unwrap();

	match Share::get(share_id).await {
		Ok(share_opt) => match share_opt {
			Some(share) => {
				if share.can_view(&session, None) {
					HttpResponse::Ok().json(share)
				} else {
					create_unauthorized_response()
				}
			}
			None => create_not_found_response(),
		},
		Err(_) => create_unauthorized_response(),
	}
}

/// Create a new share
pub async fn route_create_share(user: User, share: web::Json<UpsertShare>) -> impl Responder {
	let mut share = Share::from(share.into_inner());
	share.user_id = user.id;

	match share.insert().await {
		Ok(_) => create_created_response(&share.id),
		Err(error) => create_internal_server_error_response(Some(error)),
	}
}

/// Update an share
pub async fn route_update_share(session: Session, req: HttpRequest, updated_share: web::Json<UpsertShare>) -> impl Responder {
	let share_id = req.match_info().get("share_id").unwrap();
	let updated_share = updated_share.into_inner();

	match &session.user_id {
		Some(user_id) => match Share::get_as_user(share_id, user_id.to_string()).await {
			Ok(share_opt) => match share_opt {
				Some(mut share) => {
					if !share.can_update(&Some(session)) {
						return create_unauthorized_response();
					}

					share.type_ = updated_share.type_;
					share.data = updated_share.data;
					share.key = updated_share.key;
					share.password = updated_share.password;
					share.identifier_hash = updated_share.identifier_hash;

					match share.update().await {
						Ok(_) => create_ok_response(),
						Err(error) => create_internal_server_error_response(Some(error)),
					}
				}
				None => create_not_found_response(),
			},
			Err(_) => create_unauthorized_response(),
		},
		None => create_unauthorized_response(),
	}
}

/// Delete an share
pub async fn route_delete_share(session: Session, req: HttpRequest) -> impl Responder {
	let share_id = req.match_info().get("share_id").unwrap();

	match &session.user_id {
		Some(user_id) => match Share::get_as_user(share_id, user_id.to_string()).await {
			Ok(share_opt) => match share_opt {
				Some(share) => {
					if !share.can_delete(&Some(session)) {
						return create_unauthorized_response();
					}

					match share.delete().await {
						Ok(_) => create_ok_response(),
						Err(error) => create_internal_server_error_response(Some(error)),
					}
				}
				None => create_not_found_response(),
			},
			Err(_) => create_unauthorized_response(),
		},
		None => create_unauthorized_response(),
	}
}
