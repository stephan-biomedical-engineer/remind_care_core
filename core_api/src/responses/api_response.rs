use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use tracing::{error, warn};

use crate::services::users_service::ServiceError;

pub type ApiError = (StatusCode, Json<Value>);

pub fn internal_error(message: &str) -> ApiError
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json
        (
            json!
            (
                {"error": message}
            )
        ),
    )
}

pub fn not_found(message: &str) -> ApiError 
{
    warn!("Recurso nao encontrado: {}", message);
    (
        StatusCode::NOT_FOUND,
        Json
        (
            json!
            (
                {"error": message}
            )
        ),
    )
}

pub fn service_error(error: ServiceError, message: &str) -> ApiError
{
    match error
    {
        ServiceError::Database(err) =>
        {
            error!(error = ?err, "Database error");
            internal_error(message)
        }
        ServiceError::NotFound => not_found("User not found"),
    }
}

pub fn unauthorized(message: &str) -> ApiError
{
    warn!("Acesso nao autorizado: {}", message);
    (
        StatusCode::UNAUTHORIZED,
        Json
        (
            json!
            (
                {"error": message }
            )
        ),
    )
}

pub fn conflict(message: &str) -> ApiError
{
    warn!("Conflito de dados: {}", message);
    (
        StatusCode::CONFLICT,
        Json
        (
            json!
            (
                { "error": message }
            )
        ),
    )
}

pub fn validation_error(message: &str) -> ApiError
{
    (
        StatusCode::BAD_REQUEST,
        Json
        (
            json!
            (
                { "error": message }
            )
        ),
    )
}

