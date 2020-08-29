use actix_web::{web, HttpResponse, Responder};

use crate::database::{DatabaseEntity, DatabaseUserEntity};
use crate::web::http::*;
use crate::entities::AccessControl;
use crate::entities::user::User;
use crate::entities::collection::Collection;
use crate::web::handlers::requests::*;
use crate::web::handlers::responses::*;

/// Get all collections
pub async fn get_collections(user: User) -> impl Responder {
    match Collection::get_all_as_user(user.id.to_string()) {
		Ok(collections) => {
			let client_collections: Vec<ClientCollection> = collections.into_iter()
				.map(|c| ClientCollection::from(c)).collect();
			HttpResponse::Ok().json(client_collections)
		},
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Get extended information of an collection
pub async fn get_collection(user: Option<User>, collection_id: web::Path<String>) -> impl Responder {
    match Collection::get(&collection_id) {
		Ok(opt) => {
			match opt {
				Some(collection) => {
					if collection.user_has_access(user) {
						HttpResponse::Ok().json(ClientCollection::from(collection))
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

/// Create a new collection
pub async fn create_collection(user: User, collection: web::Json<CreateCollection>) -> impl Responder {
	let collection = Collection::new(&user.id, &collection.title);
	match collection.insert() {
		Ok(_) => create_created_response(&collection.id),
		Err(error) => create_internal_server_error_response(Some(error))
	}
}

/// Update an collection
pub async fn update_collection(user: User, collection_id: web::Path<String>, updated_collection: web::Json<UpdateCollection>) -> impl Responder {
    match Collection::get(&collection_id) {
		Ok(opt) => {
			match opt {
				Some(mut collection) => {
					if collection.user_has_access(Some(user)) {

						if let Some(title) = &updated_collection.title {
							collection.title = title.to_string();
						}
						if let Some(albums) = &updated_collection.albums {
							collection.albums = albums.to_vec();
						}
						if let Some(sharing_options) = &updated_collection.sharing {
							collection.sharing.shared = sharing_options.shared;
							// if let Some(password) = &sharing_options.password {
							// 	if password.len() == 0 {
							// 		return create_bad_request_response(Box::from("Empty password provided"));
							// 	}

							// 	collection.sharing.password_hash = Some(password.to_string());
							// }
							// else {
							// 	collection.sharing.password_hash = None;
							// }


							if sharing_options.require_password {
								// Check if a password was provided in request:
								if let Some(password) = &sharing_options.password {
									if password.len() == 0 {
										return create_bad_request_response(Box::from("Empty password provided"));
									}
	
									collection.sharing.password_hash = Some(password.to_string());
								}
								else {
									// If no password is provided in request, then there must already be one set.
									// Otherwise the request is not valid
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

/// Delete an collection
pub async fn delete_collection(user: User, collection_id: web::Path<String>) -> impl Responder {
    match Collection::get(&collection_id) {
		Ok(opt) => {
			match opt {
				Some(collection) => {
					if collection.user_has_access(Some(user)) {
						match collection.delete(){
							Ok(_) => HttpResponse::Ok().finish(),
							Err(error) => create_internal_server_error_response(Some(error))
						}
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