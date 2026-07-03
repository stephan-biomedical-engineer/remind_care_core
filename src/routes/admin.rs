use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::app::AppState;

const DEVICE_ID_PREFIX: &str = "RC";
const DEVICE_ID_LENGTH: usize = 6;
const API_KEY_LENGTH: usize = 48;

#[derive(Serialize)]
pub struct ProvisionResponse {
    pub device_id: String,
    pub api_key: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

fn generate_device_id() -> String {
    let mut rng = rand::rng();
    let hex_chars: Vec<char> = "0123456789ABCDEF".chars().collect();
    let random_part: String = (0..DEVICE_ID_LENGTH)
        .map(|_| hex_chars[rng.random_range(0..hex_chars.len())])
        .collect();

    format!("{}-{}", DEVICE_ID_PREFIX, random_part)
}

fn generate_api_key() -> String {
    let mut rng = rand::rng();
    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
        .chars()
        .collect();
    (0..API_KEY_LENGTH)
        .map(|_| charset[rng.random_range(0..charset.len())])
        .collect()
}

fn hash_api_key(api_key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    hex::encode(hasher.finalize())
}

pub async fn provision_device(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ProvisionResponse>, (StatusCode, Json<ErrorResponse>)> {
    let admin_key = headers.get("X-Admin-Secret")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    if admin_key != state.config.admin_secret_key {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                error: "Unauthorized: Invalid admin secret key".to_string(),
            }),
        ));
    }

    let device_id = generate_device_id();
    let api_key = generate_api_key();
    let api_key_hash = hash_api_key(&api_key);

    match sqlx::query("INSERT INTO devices (id, api_key_hash, is_active) VALUES ($1, $2, true)")
        .bind(&device_id)
        .bind(&api_key_hash)
        .execute(&state.pool)
        .await
    {
        Ok(_) => Ok(Json(ProvisionResponse { device_id, api_key })),
        Err(e) => {
            tracing::error!("Failed to insert device into database: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to provision device in database".to_string(),
                }),
            ))
        }
    }
}
