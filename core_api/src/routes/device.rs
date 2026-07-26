use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use validator::Validate;

use crate::models::device::*;
use crate::models::firmware::FirmwareManifest;
use crate::responses::api_response::{firmware_error, internal_error, service_error, validation_error, ApiError};
use crate::services::{device_service, firmware_service};
use crate::auth::device_extractor::AuthDevice;
use crate::auth::extractor::AuthUser;
use crate::app::AppState;

/// GET /api/v1/devices/schedule
/// A caixa baixa a agenda de medicamentos do paciente vinculado.
pub async fn get_schedule(
    auth_device: AuthDevice,
    State(state): State<AppState>,
) -> Result<Json<ScheduleResponse>, ApiError> {
    let schedule = device_service::get_schedule(&state.pool, &auth_device.device_id, auth_device.user_id)
        .await
        .map_err(|err| service_error(err, "Device not bound to any user"))?;

    Ok(Json(schedule))
}

/// POST /api/v1/devices/events
/// A caixa reporta um evento físico (abertura, fechamento, violação).
pub async fn report_event(
    auth_device: AuthDevice,
    State(state): State<AppState>,
    Json(payload): Json<DeviceEventRequest>,
) -> Result<(StatusCode, Json<DeviceEvent>), ApiError> {
    payload.validate().map_err(|_| validation_error("Invalid event payload"))?;

    let project_id = std::env::var("FCM_PROJECT_ID").unwrap_or_else(|_| "remindcare-1efbd".to_string());
    
    let event = device_service::report_event(
        &state.pool, 
        &state.fcm_manager, 
        &project_id, 
        &auth_device.device_id, 
        &payload
    )
    .await
    .map_err(|err| service_error(err, "Failed to save device event"))?;

    Ok((StatusCode::CREATED, Json(event)))
}

/// POST /api/v1/devices/heartbeat
/// A caixa envia sinal de vida + estado do hardware.
pub async fn heartbeat(
    auth_device: AuthDevice,
    State(state): State<AppState>,
    Json(payload): Json<HeartbeatRequest>,
) -> Result<Json<HeartbeatResponse>, ApiError> {
    payload.validate().map_err(|_| validation_error("Invalid heartbeat payload"))?;

    let response = device_service::process_heartbeat(
        &state.pool,
        &auth_device.device_id,
        auth_device.user_id,
        &payload,
    )
    .await
    .map_err(|err| service_error(err, "Failed to process heartbeat"))?;

    Ok(Json(response))
}

/// POST /api/v1/devices/logs
/// A caixa envia logs de erro do firmware.
pub async fn report_log(
    auth_device: AuthDevice,
    State(state): State<AppState>,
    Json(payload): Json<DeviceLogRequest>,
) -> Result<(StatusCode, Json<DeviceLog>), ApiError> {
    payload.validate().map_err(|_| validation_error("Invalid log payload"))?;

    let log = device_service::report_log(&state.pool, &auth_device.device_id, &payload)
        .await
        .map_err(|err| service_error(err, "Failed to save device log"))?;

    Ok((StatusCode::CREATED, Json(log)))
}

/// POST /api/v1/devices/bind
/// O app mobile pareia uma caixa ao usuário logado (via QR Code).
/// Protegido pelo AuthUser (JWT do app mobile), não pelo AuthDevice.
pub async fn bind_device(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<BindDeviceRequest>,
) -> Result<StatusCode, ApiError> {
    payload.validate().map_err(|_| validation_error("Invalid bind payload"))?;

    device_service::bind_device(&state.pool, auth_user.user_id, &payload)
        .await
        .map_err(|err| service_error(err, "Failed to bind device. Device not found, already bound, or deactivated."))?;

    Ok(StatusCode::NO_CONTENT)
}

/// GET /api/v1/devices/firmware
/// A caixa consulta o manifesto do firmware mais recente disponível (metadados).
pub async fn get_firmware(
    _auth_device: AuthDevice,
    State(state): State<AppState>,
) -> Result<Json<FirmwareManifest>, ApiError> {
    let release = firmware_service::get_latest(&state.pool)
        .await
        .map_err(firmware_error)?;

    Ok(Json(release.into()))
}

/// GET /api/v1/devices/firmware/download
/// A caixa baixa (streaming) o binário do firmware mais recente a partir do volume da VPS.
pub async fn get_firmware_download(
    _auth_device: AuthDevice,
    State(state): State<AppState>,
) -> Result<Response, ApiError> {
    let release = firmware_service::get_latest(&state.pool)
        .await
        .map_err(firmware_error)?;

    let path = std::path::Path::new(&state.config.firmware_dir).join(&release.filename);

    let file = tokio::fs::File::open(&path).await.map_err(|err| {
        tracing::error!(error = ?err, path = ?path, "Failed to open firmware binary");
        internal_error("Firmware binary unavailable")
    })?;

    let stream = tokio_util::io::ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let headers = [
        (header::CONTENT_TYPE, "application/octet-stream".to_string()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", release.filename),
        ),
    ];

    Ok((headers, body).into_response())
}

/// GET /api/v1/devices/me
/// Retorna o dispositivo pareado do usuário (app mobile)
pub async fn get_my_device(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<PublicDevice>, ApiError> {
    let device = device_service::get_user_device(&state.pool, auth_user.user_id)
        .await
        .map_err(|err| service_error(err, "Device not found"))?;

    Ok(Json(device.into()))
}
