use axum::{extract::{Path, State}, http::StatusCode, Json};
use validator::Validate;

use crate::models::medicine::{Medicine, CreateMedicineRequest, UpdateMedicineRequest, MedicineLog, CreateMedicineLogRequest};
use crate::responses::api_response::{service_error, validation_error, ApiError};
use crate::services::medicine_service;
use crate::auth::extractor::AuthUser;
use crate::app::AppState;

pub async fn create_medicine(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateMedicineRequest>,
) -> Result<(StatusCode, Json<Medicine>), ApiError> {
    payload.validate().map_err(|_| validation_error("Invalid medicine payload"))?;
    
    let medicine = medicine_service::create_medicine(&state.pool, auth_user.user_id, &payload)
        .await
        .map_err(|err| service_error(err, "Failed to create medicine"))?;
        
    Ok((StatusCode::CREATED, Json(medicine)))
}

pub async fn list_medicines(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<Medicine>>, ApiError> {
    let medicines = medicine_service::list_medicines(&state.pool, auth_user.user_id)
        .await
        .map_err(|err| service_error(err, "Failed to list medicines"))?;
        
    Ok(Json(medicines))
}

pub async fn get_medicine(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Medicine>, ApiError> {
    let medicine = medicine_service::get_medicine(&state.pool, id, auth_user.user_id)
        .await
        .map_err(|err| service_error(err, "Failed to fetch medicine"))?;
        
    Ok(Json(medicine))
}

pub async fn update_medicine(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateMedicineRequest>,
) -> Result<Json<Medicine>, ApiError> {
    payload.validate().map_err(|_| validation_error("Invalid medicine update payload"))?;
    
    let medicine = medicine_service::update_medicine(&state.pool, id, auth_user.user_id, &payload)
        .await
        .map_err(|err| service_error(err, "Failed to update medicine"))?;
        
    Ok(Json(medicine))
}

pub async fn delete_medicine(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    medicine_service::delete_medicine(&state.pool, id, auth_user.user_id)
        .await
        .map_err(|err| service_error(err, "Failed to delete medicine"))?;
        
    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_log(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(payload): Json<CreateMedicineLogRequest>,
) -> Result<(StatusCode, Json<MedicineLog>), ApiError> {
    payload.validate().map_err(|_| validation_error("Invalid log payload"))?;
    
    let log = medicine_service::log_medicine_opened(&state.pool, auth_user.user_id, &payload)
        .await
        .map_err(|err| service_error(err, "Failed to save medicine log"))?;
        
    Ok((StatusCode::CREATED, Json(log)))
}

pub async fn get_today_logs(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> Result<Json<Vec<MedicineLog>>, ApiError> {
    let logs = medicine_service::get_today_logs(&state.pool, auth_user.user_id)
        .await
        .map_err(|err| service_error(err, "Failed to fetch today logs"))?;
        
    Ok(Json(logs))
}
