use std::path::Path;

use sqlx::PgPool;

use crate::models::firmware::{FirmwareRelease, PublishFirmwareRequest};
use crate::repositories::firmware_repository::FirmwareRepository;

pub enum FirmwareError {
    Database(sqlx::Error),
    NotFound,
    Conflict(String),
    FileMissing(String),
}

/// Release mais recente disponível para os dispositivos.
pub async fn get_latest(pool: &PgPool) -> Result<FirmwareRelease, FirmwareError> {
    FirmwareRepository::get_latest(pool)
        .await
        .map_err(FirmwareError::Database)?
        .ok_or(FirmwareError::NotFound)
}

/// Publica um novo release. Valida que:
/// 1. O binário referenciado já existe no diretório de firmware (volume da VPS).
/// 2. O build_number é estritamente maior que o atual (impede downgrade acidental).
pub async fn publish(
    pool: &PgPool,
    firmware_dir: &str,
    req: &PublishFirmwareRequest,
) -> Result<FirmwareRelease, FirmwareError> {
    let path = Path::new(firmware_dir).join(&req.filename);
    if !path.is_file() {
        return Err(FirmwareError::FileMissing(format!(
            "Firmware binary '{}' not found in firmware directory",
            req.filename
        )));
    }

    if let Some(current_max) = FirmwareRepository::get_max_build_number(pool)
        .await
        .map_err(FirmwareError::Database)?
    {
        if req.build_number <= current_max {
            return Err(FirmwareError::Conflict(format!(
                "build_number {} must be greater than the current {}",
                req.build_number, current_max
            )));
        }
    }

    FirmwareRepository::insert_release(
        pool,
        &req.version,
        req.build_number,
        &req.filename,
        &req.sha256,
        req.release_notes.as_deref(),
    )
    .await
    .map_err(FirmwareError::Database)
}
