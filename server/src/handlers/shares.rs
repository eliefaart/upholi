use super::auth_share_for_session;
use crate::database::{self, *};
use crate::model::{Session, Share};
use crate::UserId;
use anyhow::Result;
use axum::{extract::Path, http::StatusCode, Json};
use upholi_lib::http::request::*;
use upholi_lib::passwords::{hash_password, verify_password_hash};

pub async fn is_authorized_for_share(Path(id): Path<String>, session: Session) -> StatusCode {
    match session.shares.contains(&id) {
        true => StatusCode::OK,
        false => StatusCode::UNAUTHORIZED,
    }
}

/// Attempt to authorize to a share
pub async fn authorize_share(
    session: Session,
    Path(id): Path<String>,
    Json(credentials): Json<AuthorizeShareRequest>,
) -> Result<StatusCode, StatusCode> {
    let already_authorized = session.shares.contains(&id);

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
            auth_share_for_session(session, &share.id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok(StatusCode::OK)
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Create or update a share
pub async fn create_share(UserId(user_id): UserId, Json(share): Json<UpsertShareRequest>) -> Result<StatusCode, StatusCode> {
    let password_phc = hash_password(&share.password).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let item_ids_for_share = [vec![share.id.clone()], share.items].concat();

    let share = Share {
        id: share.id.clone(),
        user_id: user_id.clone(),
        password_phc,
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

/// Delete a share
pub async fn delete_share(UserId(user_id): UserId, Path(id): Path<String>) -> Result<StatusCode, StatusCode> {
    remove_items_from_share(&id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    database::delete_share(&user_id, &id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::OK)
}
