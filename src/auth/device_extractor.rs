use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use sha2::{Digest, Sha256};

use crate::app::AppState;
use crate::responses::api_response::{ApiError, unauthorized};

/// Struct extraída de requisições autenticadas por dispositivos IoT.
/// Diferente do AuthUser (JWT), aqui usamos API Key estática.
pub struct AuthDevice {
    pub device_id: String,
    pub user_id: Option<i32>,
}

impl FromRequestParts<AppState> for AuthDevice {
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Extrair o header Authorization: Bearer <api_key>
        let auth_header = parts.headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let api_key = match auth_header {
            Some(value) if value.starts_with("Bearer ") => {
                value.trim_start_matches("Bearer ").to_string()
            }
            _ => {
                return Err(unauthorized("Missing or invalid device Authorization header"));
            }
        };

        // 2. Calcular o hash SHA-256 da API Key recebida
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let key_hash = hex::encode(hasher.finalize());

        // 3. Buscar no banco o dispositivo correspondente
        let device = sqlx::query_as!(
            crate::models::device::Device,
            r#"
            SELECT id, user_id, api_key_hash, firmware_version, last_heartbeat_at, is_active, created_at
            FROM devices WHERE api_key_hash = $1
            "#,
            key_hash
        )
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| unauthorized("Device authentication failed"))?;

        match device {
            Some(d) if d.is_active => {
                Ok(AuthDevice {
                    device_id: d.id,
                    user_id: d.user_id,
                })
            }
            Some(_) => Err(unauthorized("Device is deactivated")),
            None => Err(unauthorized("Unknown device")),
        }
    }
}
