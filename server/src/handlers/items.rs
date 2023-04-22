use crate::database;
use crate::model::{EncryptedData, Session};
use crate::UserId;
use anyhow::Result;
use axum::{extract::Path, http::StatusCode, Json};
use upholi_lib::http::request::DeleteManyRequest;

pub async fn get_item_ids(UserId(user_id): UserId) -> Result<Json<Vec<String>>, StatusCode> {
    match database::get_item_ids::<EncryptedData>(&user_id).await {
        Ok(ids) => Ok(Json(ids)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_item(session: Session, Path(id): Path<String>) -> Result<Json<EncryptedData>, StatusCode> {
    match database::get_item(&id, &session).await {
        Ok(option) => match option {
            Some(value) => Ok(Json(value)),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn set_item(
    UserId(user_id): UserId,
    Path(id): Path<String>,
    Json(item): Json<EncryptedData>,
) -> Result<StatusCode, StatusCode> {
    match database::upsert_item(&id, item, &user_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_item(UserId(user_id): UserId, Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
    delete_items(UserId(user_id), Json(DeleteManyRequest { ids: vec![id] })).await
}

pub async fn delete_items(
    UserId(user_id): UserId,
    Json(request): Json<DeleteManyRequest>,
) -> Result<StatusCode, StatusCode> {
    match database::delete_items::<EncryptedData>(&request.ids, &user_id).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
