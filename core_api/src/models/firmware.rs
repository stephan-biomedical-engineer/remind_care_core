use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ─── Entidade do Banco ───

#[derive(Debug, sqlx::FromRow)]
pub struct FirmwareRelease {
    pub id: i32,
    pub version: String,
    pub build_number: i32,
    pub filename: String,
    pub sha256: String,
    pub release_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

// ─── Response (manifesto entregue à caixa) ───

#[derive(Debug, Serialize)]
pub struct FirmwareManifest {
    pub version: String,
    pub build_number: i32,
    pub sha256: String,
    pub release_notes: Option<String>,
}

impl From<FirmwareRelease> for FirmwareManifest {
    fn from(release: FirmwareRelease) -> Self {
        Self {
            version: release.version,
            build_number: release.build_number,
            sha256: release.sha256,
            release_notes: release.release_notes,
        }
    }
}

// ─── Request (publicação de um novo release pelo Arquiteto) ───

#[derive(Debug, Deserialize, Validate)]
pub struct PublishFirmwareRequest {
    #[validate(length(min = 1, max = 20))]
    pub version: String,
    pub build_number: i32,
    #[validate(length(min = 1, max = 255))]
    pub filename: String,
    #[validate(length(equal = 64))]
    pub sha256: String,
    pub release_notes: Option<String>,
}
