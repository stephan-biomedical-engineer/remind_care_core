use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;

use crate::app::AppState;
use crate::models::auth::{
    AccessTokenResponse,
    AuthResponse,
    LoginRequest,
    LogoutRequest,
    RefreshRequest,
    RegisterRequest,
};
use crate::models::user::PublicUser;
use crate::responses::api_response::{ApiError, conflict, internal_error, unauthorized, validation_error};
use crate::services::auth_service::{self, AuthError};

fn auth_error(err: AuthError) -> ApiError
{
    match err
    {
        AuthError::Database(e) =>
        {
            tracing::error!(error = ?e, "Database error");
            internal_error("Database error")
        }
        AuthError::InvalidCredentials => unauthorized("Invalid credentials"),
        AuthError::UserAlreadyExists => conflict("User already exists"),
        AuthError::HashingError => internal_error("Failed to hash password"),
        AuthError::TokenCreationError => internal_error("Failed to create token"),
    }
}

pub async fn register
(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<PublicUser>), ApiError>
{
    payload.validate()
        .map_err(|_| validation_error("Invalid register payload"))?;

    let user = auth_service::register(&state.pool, payload)
        .await
        .map_err(auth_error)?;

    Ok((StatusCode::CREATED, Json(user.into())))
}

pub async fn login
(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError>
{
    payload.validate()
        .map_err(|_| validation_error("Invalid login payload"))?;

    let response = auth_service::login
    (
        &state.pool,
        &state.config.jwt_secret,
        payload,
    )
        .await
        .map_err(auth_error)?;

    Ok(Json(response))
}

pub async fn refresh
(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<AccessTokenResponse>, ApiError>
{
    payload.validate()
        .map_err(|_| validation_error("Invalid refresh payload"))?;

    let response = auth_service::refresh
    (
        &state.pool,
        &state.config.jwt_secret,
        payload,
    )
        .await
        .map_err(auth_error)?;

    Ok(Json(response))
}

pub async fn logout
(
    State(state): State<AppState>,
    Json(payload): Json<LogoutRequest>,
) -> Result<StatusCode, ApiError>
{
    payload.validate()
        .map_err(|_| validation_error("Invalid logout payload"))?;

    auth_service::logout(&state.pool, payload)
        .await
        .map_err(auth_error)?;

    Ok(StatusCode::NO_CONTENT)
}
