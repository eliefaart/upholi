use crate::database::*;
use crate::model::{Session, TextItem, TextItemData};
use crate::UserId;
use anyhow::Result;
use axum::{extract::Path, http::StatusCode, Json};

pub async fn get_text_keys(UserId(user_id): UserId) -> Result<Json<Vec<String>>, StatusCode> {
	match get_item_keys::<TextItemData>(&user_id).await {
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

pub async fn delete_text(UserId(user_id): UserId, Path(key): Path<String>) -> Result<StatusCode, StatusCode> {
	match delete_item::<TextItemData>(&key, &user_id).await {
		Ok(_) => Ok(StatusCode::OK),
		Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}
}
