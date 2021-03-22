use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

use crate::{database::{DatabaseEntity, DatabaseUserEntity}, passwords::hash_password, web::cookies::create_session_cookie};
use crate::web::http::*;
use crate::entities::AccessControl;
use crate::entities::user::User;
use crate::entities::session::Session;
use crate::entities::collection::Collection;
use crate::web::handlers::requests::*;
use crate::web::handlers::responses::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RotateTokenResult {
	pub token: String
}

/// Get all collections
pub async fn get_collections(user: User) -> impl Responder {
    match Collection::get_all_as_user(user.id.to_string()) {
		Ok(collections) => {
			let client_collections: Vec<ClientCollection> = collections.into_iter()
				.map(|collection| ClientCollection::from(&collection)).collect();
			HttpResponse::Ok().json(client_collections)
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Get extended information of an collection
pub async fn get_collection(session: Session, collection_id: web::Path<String>) -> impl Responder {
	match Collection::get(&collection_id) {
		Ok(opt) => {
			match opt {
				Some(collection) => {
					if collection.can_view(&Some(session)) {
						HttpResponse::Ok().json(ClientCollection::from(&collection))
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

/// Get a single shared collection collection by its token
pub async fn get_collections_by_share_token(session: Option<Session>, token: web::Path<String>) -> impl Responder {
	match Collection::get_by_share_token(&token) {
		Ok(opt) => {
			match opt {
				Some(collection) => {
					if collection.can_view(&session) {
						HttpResponse::Ok().json(ClientCollection::from(&collection))
					}
					else {
						// Password required?
						create_unauthorized_response()
					}
				},
				None => create_not_found_response()
			}
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Create a new collection
pub async fn create_collection(user: User, collection: web::Json<CreateCollection>) -> impl Responder {
	let collection = Collection::new(&user.id, &collection.title);
	match collection.insert() {
		Ok(_) => create_created_response(&collection.id),
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Update an collection
pub async fn update_collection(session: Session, collection_id: web::Path<String>, updated_collection: web::Json<UpdateCollection>) -> impl Responder {
	handle_collection_update_operation(session, collection_id, move |collection|{
		if let Some(title) = &updated_collection.title {
			collection.title = title.to_string();
		}
		if let Some(albums) = &updated_collection.albums {
			collection.albums = albums.to_vec();
		}
		if let Some(sharing_options) = &updated_collection.sharing {
			// TODO: If password changes, then revoke access from all sessions that have been granted access to current collection

			if sharing_options.require_password {
				// Check if a password was provided in request:
				// if it is, then update it in database
				// if it is not provided, then it must already exist in database, otherwise request is invalid
				if let Some(password) = &sharing_options.password {
					if password.len() == 0 {
						return create_bad_request_response(Box::from("Empty password provided"));
					}

					match hash_password(&password, &collection.id) {
						Ok(password_hash) => collection.sharing.password_hash = Some(password_hash),
						Err(error) => {
							return create_internal_server_error_response(Some(error));
						}
					}
				}
				else {
					if collection.sharing.password_hash.is_none() {
						create_bad_request_response(Box::from("Missing password in request"));
					}
				}
			}
			else {
				collection.sharing.password_hash = None;
			}
		}

		match collection.update(){
			Ok(_) => HttpResponse::Ok().finish(),
			Err(error) => create_internal_server_error_response(Some(error))
		}
	}).await
}

/// Delete an collection
pub async fn delete_collection(session: Session, collection_id: web::Path<String>) -> impl Responder {
	handle_collection_update_operation(session, collection_id, |collection| {
		match collection.delete(){
			Ok(_) => HttpResponse::Ok().finish(),
			Err(error) => create_internal_server_error_response(Some(error))
		}
	}).await
}

/// Rotate the token with which a collection may be accessed by clients other than the user that owns a collection
pub async fn rotate_collection_share_token(session: Session, collection_id: web::Path<String>) -> impl Responder {
	handle_collection_update_operation(session, collection_id, |collection| {
		collection.rotate_share_token();

		match collection.update(){
			Ok(_) => HttpResponse::Ok().json(RotateTokenResult{token: collection.sharing.token.to_string()}),
			Err(error) => create_internal_server_error_response(Some(error))
		}
	}).await
}

/// Grant a session access to a password-protected collection, if the password is correct.
pub async fn authenticate_to_collection(session_opt: Option<Session>, token: web::Path<String>, authenticate_request: web::Json<AuthenticateToCollection>) -> impl Responder {
	let request_has_session = session_opt.is_some();

	match &authenticate_request.password {
		Some(password) => {
			match Collection::get_by_share_token(&token) {
				Ok(opt) => {
					match opt {
						Some(collection) => {
							match collection.password_correct(&password) {
								true => {
									match get_session_or_create_new(session_opt) {
										Ok(mut session) => {
											// Authenticate this session for the collection
											if !session.authenticated_for_collection_tokens.contains(&collection.sharing.token) {
												session.authenticated_for_collection_tokens.push(collection.sharing.token);
												if let Err(error) = session.update() {
													return create_internal_server_error_response(Some(error));
												}
											}

											let mut response = HttpResponse::Ok()
												.finish();

											// Append the new session cookie to the response
											if !request_has_session {
												let cookie = create_session_cookie(&session);

												if let Err(error) = response.add_cookie(&cookie) {
													return create_internal_server_error_response(Some(Box::new(error)));
												}
											}

											response
										},
										Err(error) => create_internal_server_error_response(Some(error))
									}
								},
								false => {
									create_unauthorized_response()
								}
							}
						},
						None => create_not_found_response()
					}
				},
				Err(error) => create_internal_server_error_response(Some(error))
			}
		},
		None => create_bad_request_response(Box::from("No password"))
	}
}

/// Perform some action on a collection, if it exists and the given user has access to it.
/// The action (fn_collection_action) must return an appropriate HttpResponse
async fn handle_collection_update_operation<F>(session: Session, collection_id: web::Path<String>, fn_collection_action: F) -> impl Responder
	where F: Fn(&mut Collection) -> HttpResponse {
	match Collection::get(&collection_id) {
		Ok(opt) => {
			match opt {
				Some(mut collection) => {
					if collection.can_update(&Some(session)) {
						// Call the action for current collection and return its result
						fn_collection_action(&mut collection)
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