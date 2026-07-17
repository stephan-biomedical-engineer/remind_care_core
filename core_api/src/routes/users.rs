use axum::{extract::{Path, State}, http::StatusCode, Json};
use validator::Validate;

use crate::models::user::{PublicUser, UpdateUserRequest, UpdateFcmTokenRequest};
use crate::responses::api_response::{service_error, unauthorized, validation_error, ApiError};
use crate::services::users_service;
use crate::auth::extractor::AuthUser;
use crate::app::AppState;

pub async fn list_users
    (
        _auth_user: AuthUser,
        State(state): State<AppState>,
    ) -> Result<Json<Vec<PublicUser>>, ApiError>
{
    let users = users_service::list_users(&state.pool)
        .await
        .map_err(|err| service_error(err, "Failed to fetch users"))?;

    let public_users = users
        .into_iter()
        .map(PublicUser::from)
        .collect();

    Ok(Json(public_users))
}

pub async fn get_user
    (
        auth_user: AuthUser,
        State(state): State<AppState>,
        Path(id): Path<uuid::Uuid>,
    ) -> Result<Json<PublicUser>, ApiError>
{
    if auth_user.user_id != id
    {
        return Err(unauthorized("You can only view your own account"));
    }

    let user = users_service::get_user(&state.pool, id)
        .await
        .map_err(|err| service_error(err, "Failed to fetch user"))?;

    Ok(Json(user.into()))
}

pub async fn update_user
    (
        auth_user: AuthUser,
        State(state): State<AppState>,
        Path(id): Path<uuid::Uuid>,
        Json(payload): Json<UpdateUserRequest>,
    ) -> Result<Json<PublicUser>, ApiError>
{
    payload.validate()
        .map_err(|_| validation_error("Invalid update payload"))?;
        
    if auth_user.user_id != id
    {
        return Err(unauthorized("You can only update your own account"));
    }

    let user = users_service::update_user(&state.pool, id, payload.name)
        .await
        .map_err(|err| service_error(err, "Failed to update user"))?;

    Ok(Json(user.into()))
}

pub async fn delete_user
    (
        auth_user: AuthUser,
        State(state): State<AppState>,
        Path(id): Path<uuid::Uuid>,
    ) -> Result<StatusCode, ApiError>
{
    if auth_user.user_id != id
    {
        return Err(unauthorized("You can only delete your own account"));
    }

    users_service::delete_user(&state.pool, id)
        .await
        .map_err(|err| service_error(err, "Failed to delete user"))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_user_fcm_token
    (
        auth_user: AuthUser,
        State(state): State<AppState>,
        Json(payload): Json<UpdateFcmTokenRequest>,
    ) -> Result<Json<PublicUser>, ApiError>
{
    payload.validate()
        .map_err(|_| validation_error("Invalid fcm_token payload"))?;
        
    let user = users_service::update_fcm_token(&state.pool, auth_user.user_id, payload.fcm_token)
        .await
        .map_err(|err| service_error(err, "Failed to update FCM token"))?;

    Ok(Json(user.into()))
}