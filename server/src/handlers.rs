use crate::database::{self, *};
use crate::model::{FileItemData, Session, Share, TextItem, TextItemData, User};
use crate::storage::{self, init_storage_for_user, store_file};
use crate::{create_sesson_cookie, OptionalSession, UserId};
use anyhow::{anyhow, Result};
use axum::extract::Multipart;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use tower_cookies::Cookies;
use upholi_lib::http::{request::*, response::*};
use upholi_lib::ids::id;
use upholi_lib::passwords::{hash_password, verify_password_hash};

struct MultipartEntry {
	pub key: String,
	pub bytes: Vec<u8>,
}

pub async fn get_user(UserId(_): UserId) -> StatusCode {
	StatusCode::OK
}

pub async fn create_user(
	cookies: Cookies,
	session: OptionalSession,
	Json(user_info): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
	let session = session.0;
	let result = handler_create_user(&user_info)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	auth_user_for_session(session, cookies, &result.id)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok((StatusCode::CREATED, Json(result)))
}

pub async fn handler_create_user(user_info: &CreateUserRequest) -> Result<CreatedResult> {
	if get_user_by_username(&user_info.username).await?.is_some() {
		Err(anyhow!("A user with this username already exists."))
	} else {
		let password_phc = hash_password(&user_info.password)?;
		let user_id = id();
		let user = User {
			id: user_id.clone(),
			username: user_info.username.clone(),
			password_phc,
		};
		insert_user(&user).await?;
		init_storage_for_user(&user).await?;
		Ok(CreatedResult { id: user_id })
	}
}

pub async fn authenticate_user(
	cookies: Cookies,
	session: OptionalSession,
	Json(credentials): Json<AuthenticateUserRequest>,
) -> impl IntoResponse {
	let session = session.0;
	let user = get_user_by_username(&credentials.username)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
		.ok_or(StatusCode::NOT_FOUND)?;

	let password_correct = verify_password_hash(&credentials.password, &user.password_phc);
	if password_correct {
		auth_user_for_session(session, cookies, &user.id)
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
		Ok(StatusCode::OK)
	} else {
		Err(StatusCode::UNAUTHORIZED)
	}
}

pub async fn is_authorized_for_share(Path(id): Path<String>, session: Session) -> StatusCode {
	match session.shares.contains(&id) {
		true => StatusCode::OK,
		false => StatusCode::UNAUTHORIZED,
	}
}

pub async fn authorize_share(
	cookies: Cookies,
	session: OptionalSession,
	Path(id): Path<String>,
	Json(credentials): Json<AuthorizeShareRequest>,
) -> Result<StatusCode, StatusCode> {
	let session = session.0;
	let already_authorized = session.is_some() && session.as_ref().unwrap().shares.contains(&id);

	if already_authorized {
		// This session is already authorized to this share; we won't verify the provided password.
		Ok(StatusCode::OK)
	} else {
		let share = database::get_share(&id)
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
			.ok_or(StatusCode::NOT_FOUND)?;

		let password_correct = verify_password_hash(&credentials.password, &share.password_phc);
		if password_correct {
			auth_share_for_session(session, cookies, &share.id)
				.await
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
			Ok(StatusCode::OK)
		} else {
			Err(StatusCode::UNAUTHORIZED)
		}
	}
}

pub async fn get_share(Path(id): Path<String>) -> Result<Json<GetShareResult>, StatusCode> {
	let share = database::get_share(&id)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
		.ok_or(StatusCode::NOT_FOUND)?;

	Ok(Json(GetShareResult {
		base64: share.data.base64,
		nonce: share.data.nonce,
	}))
}

pub async fn create_share(UserId(user_id): UserId, Json(share): Json<UpsertShareRequest>) -> Result<StatusCode, StatusCode> {
	let password_phc = hash_password(&share.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	let item_ids_for_share = share.items;
	let share = Share {
		id: share.id.clone(),
		user_id,
		password_phc,
		data: TextItemData {
			base64: share.base64,
			nonce: share.nonce,
		},
	};

	upsert_share(&share).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	set_items_for_share(&share.id, &item_ids_for_share)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	remove_authorizations_for_share(&share.id)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	Ok(StatusCode::OK)
}

pub async fn delete_share(UserId(user_id): UserId, Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
	database::delete_share(&user_id, &id)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	remove_items_from_share(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	Ok(StatusCode::OK)
}

pub async fn get_text_keys(UserId(user_id): UserId) -> Result<Json<Vec<String>>, StatusCode> {
	match get_item_keys::<TextItemData>(&user_id).await {
		Ok(keys) => Ok(Json(keys)),
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}

pub async fn get_file_keys(UserId(user_id): UserId) -> Result<Json<Vec<String>>, StatusCode> {
	match get_item_keys::<FileItemData>(&user_id).await {
		Ok(keys) => Ok(Json(keys)),
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}

pub async fn get_text(session: Session, Path(key): Path<String>) -> Result<Json<TextItem>, StatusCode> {
	match get_item(&key, &session).await {
		Ok(option) => match option {
			Some(value) => Ok(Json(TextItem::from_data(key, value))),
			None => Err(StatusCode::NOT_FOUND),
		},
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}
pub async fn get_text_multiple(UserId(user_id): UserId, Path(key_prefix): Path<String>) -> Result<Json<Vec<TextItem>>, StatusCode> {
	match get_items_with_prefix::<TextItemData>(&key_prefix, &user_id).await {
		Ok(items) => Ok(Json(items)),
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}

pub async fn get_file(session: Session, Path(key): Path<String>) -> Result<Vec<u8>, StatusCode> {
	match get_item::<FileItemData>(&key, &session).await {
		Ok(option) => match option {
			Some(file) => {
				let file = crate::storage::get_file(&key, &file.container)
					.await
					.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
					.ok_or(StatusCode::NOT_FOUND)?;
				Ok(file)
			}
			None => Err(StatusCode::NOT_FOUND),
		},
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}

pub async fn set_text(
	UserId(user_id): UserId,
	Path(key): Path<String>,
	Json(text): Json<TextItemData>,
) -> Result<StatusCode, StatusCode> {
	match upsert_item(&key, text, &user_id).await {
		Ok(_) => Ok(StatusCode::OK),
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}

pub async fn set_files(UserId(user_id): UserId, multipart: Multipart) -> Result<StatusCode, StatusCode> {
	let multipart_entries = get_multipart_entries(multipart)
		.await
		.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

	for entry in multipart_entries {
		let file_id = id();
		let file = FileItemData {
			file_id: file_id.clone(),
			container: user_id.clone(),
		};

		store_file(&entry.key, &user_id, &entry.bytes)
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

		upsert_item(&entry.key, file, &user_id)
			.await
			.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
	}

	Ok(StatusCode::OK)
}

pub async fn delete_text(UserId(user_id): UserId, Path(key): Path<String>) -> Result<StatusCode, StatusCode> {
	match delete_item::<TextItemData>(&key, &user_id).await {
		Ok(_) => Ok(StatusCode::OK),
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}

pub async fn delete_file(UserId(user_id): UserId, Path(key): Path<String>) -> Result<StatusCode, StatusCode> {
	match delete_item::<FileItemData>(&key, &user_id).await {
		Ok(_) => match storage::delete_file(&key, &user_id).await {
			Ok(()) => Ok(StatusCode::OK),
			Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
		},
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}

async fn get_multipart_entries(mut multipart: Multipart) -> Result<Vec<MultipartEntry>> {
	let mut entries: Vec<MultipartEntry> = vec![];

	while let Some(field) = multipart.next_field().await? {
		let key = field.name().unwrap().to_string();
		let bytes = field.bytes().await.unwrap();

		entries.push(MultipartEntry {
			key,
			bytes: bytes.to_vec(),
		});
	}

	Ok(entries)
}

/// Grant the current session access to given share ID. If no session exists, one is created.
async fn auth_share_for_session(existing_session: Option<Session>, cookies: Cookies, share_id: &str) -> Result<()> {
	let mut session = get_or_create_session(existing_session);

	if !session.shares.iter().any(|id| id == share_id) {
		session.shares.push(share_id.to_string());
		upsert_session(&session).await?
	}

	set_cookie(cookies, &session.id);
	Ok(())
}

/// Log given user ID into current session. If no session exists, one is created.
async fn auth_user_for_session(existing_session: Option<Session>, cookies: Cookies, user_id: &str) -> Result<()> {
	let mut session = get_or_create_session(existing_session);

	let some_user_id = Some(user_id.to_string());
	if session.user_id != some_user_id {
		session.user_id = some_user_id;
		upsert_session(&session).await?
	}

	set_cookie(cookies, &session.id);
	Ok(())
}

/// Get the session contained within the Option, otherwise create a new one.
fn get_or_create_session(existing_session: Option<Session>) -> Session {
	existing_session.unwrap_or_else(|| {
		let session_id = id();
		Session {
			id: session_id.clone(),
			user_id: None,
			shares: vec![],
		}
	})
}

/// Write the session cookie to the cookie container, ensuring it will be included in the response headers.
fn set_cookie(cookies: Cookies, session_id: &str) {
	let session_cookie = create_sesson_cookie(session_id.to_string());
	cookies.add(session_cookie);
}
