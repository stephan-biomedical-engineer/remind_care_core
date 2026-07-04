use sqlx::PgPool;

use crate::models::medicine::{Medicine, CreateMedicineRequest, UpdateMedicineRequest, MedicineLog, CreateMedicineLogRequest};
use crate::repositories::medicine_repository::MedicineRepository;
use crate::services::users_service::ServiceError;

pub async fn create_medicine(pool: &PgPool, user_id: i32, req: &CreateMedicineRequest) -> Result<Medicine, ServiceError> {
    MedicineRepository::create(pool, user_id, req)
        .await
        .map_err(ServiceError::Database)
}

pub async fn list_medicines(pool: &PgPool, user_id: i32) -> Result<Vec<Medicine>, ServiceError> {
    MedicineRepository::find_by_user(pool, user_id)
        .await
        .map_err(ServiceError::Database)
}

pub async fn get_medicine(pool: &PgPool, id: i32, user_id: i32) -> Result<Medicine, ServiceError> {
    MedicineRepository::find_by_id_and_user(pool, id, user_id)
        .await
        .map_err(ServiceError::Database)?
        .ok_or(ServiceError::NotFound)
}

pub async fn update_medicine(pool: &PgPool, id: i32, user_id: i32, req: &UpdateMedicineRequest) -> Result<Medicine, ServiceError> {
    MedicineRepository::update(pool, id, user_id, req)
        .await
        .map_err(ServiceError::Database)?
        .ok_or(ServiceError::NotFound)
}

pub async fn delete_medicine(pool: &PgPool, id: i32, user_id: i32) -> Result<(), ServiceError> {
    let rows = MedicineRepository::delete(pool, id, user_id)
        .await
        .map_err(ServiceError::Database)?;
    if rows == 0 {
        return Err(ServiceError::NotFound);
    }
    Ok(())
}

pub async fn log_medicine_opened(pool: &PgPool, user_id: i32, req: &CreateMedicineLogRequest) -> Result<MedicineLog, ServiceError> {
    // Check if medicine belongs to user
    let _ = get_medicine(pool, req.medicine_id, user_id).await?;
    
    MedicineRepository::create_log(pool, user_id, req.medicine_id, &req.situation)
        .await
        .map_err(ServiceError::Database)
}

pub async fn get_today_logs(pool: &PgPool, user_id: i32) -> Result<Vec<MedicineLog>, ServiceError> {
    MedicineRepository::get_logs_today(pool, user_id)
        .await
        .map_err(ServiceError::Database)
}
