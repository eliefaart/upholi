use crate::model::{File, Session};
use crate::storage::store_file;
use crate::UserId;
use crate::{database::*, storage};
use anyhow::Result;
use axum::extract::Multipart;
use axum::{extract::Path, http::StatusCode, Json};
use upholi_lib::ids::id;

struct MultipartEntry {
    pub name: String,
    pub bytes: Vec<u8>,
}

pub async fn get_file_ids(UserId(user_id): UserId) -> Result<Json<Vec<String>>, StatusCode> {
    match get_item_ids::<File>(&user_id).await {
        Ok(ids) => Ok(Json(ids)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_file(session: Session, Path(id): Path<String>) -> Result<Vec<u8>, StatusCode> {
    match get_item::<File>(&id, &session).await {
        Ok(option) => match option {
            Some(file) => {
                let file = crate::storage::get_file(&id, &file.container)
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

pub async fn set_files(UserId(user_id): UserId, multipart: Multipart) -> Result<StatusCode, StatusCode> {
    let multipart_entries = get_multipart_entries(multipart)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    for entry in multipart_entries {
        let file_id = id();
        let file = File {
            file_id: file_id.clone(),
            container: user_id.clone(),
        };

        store_file(&entry.name, &user_id, &entry.bytes)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        upsert_item(&entry.name, file, &user_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::OK)
}

pub async fn delete_file(UserId(user_id): UserId, Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
    match delete_item::<File>(&id, &user_id).await {
        Ok(_) => match storage::delete_file(&id, &user_id).await {
            Ok(()) => Ok(StatusCode::OK),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_multipart_entries(mut multipart: Multipart) -> Result<Vec<MultipartEntry>> {
    let mut entries: Vec<MultipartEntry> = vec![];

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap().to_string();
        let bytes = field.bytes().await.unwrap();

        entries.push(MultipartEntry {
            name,
            bytes: bytes.to_vec(),
        });
    }

    Ok(entries)
}
