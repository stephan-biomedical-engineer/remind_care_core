use chrono::DateTime;
use sqlx::PgPool;

use crate::models::device::*;
use crate::repositories::device_repository::DeviceRepository;
use crate::services::users_service::ServiceError;

pub async fn get_schedule(pool: &PgPool, device_id: &str, user_id: Option<uuid::Uuid>) -> Result<ScheduleResponse, ServiceError> {
    let user_id = user_id.ok_or(ServiceError::NotFound)?; // Dispositivo não pareado

    let schedule = DeviceRepository::get_schedule(pool, user_id)
        .await
        .map_err(ServiceError::Database)?;

    Ok(ScheduleResponse {
        device_id: device_id.to_string(),
        schedule,
    })
}

pub async fn report_event(pool: &PgPool, device_id: &str, req: &DeviceEventRequest) -> Result<DeviceEvent, ServiceError> {
    let event_timestamp = DateTime::from_timestamp(req.timestamp, 0)
        .ok_or(ServiceError::NotFound)?; // timestamp inválido

    let event = DeviceRepository::insert_event(pool, device_id, &req.event_type, event_timestamp, req.metadata.clone())
        .await
        .map_err(ServiceError::Database)?;

    // Tradução de Telemetria (Hardware) -> Evento Clínico (Aplicação)
    if req.event_type == "medication_missed" || req.event_type == "medication_taken" {
        if let Some(meta) = &req.metadata {
            if let Some(med_id_val) = meta.get("medication_id") {
                if let Some(med_id_str) = med_id_val.as_str() {
                    if let Ok(medication_id) = uuid::Uuid::parse_str(med_id_str) {
                        // Descobre o dono da caixa
                        if let Ok(Some(user_id)) = DeviceRepository::get_device_owner(pool, device_id).await {
                            let situation = if req.event_type == "medication_taken" {
                                "onTime"
                            } else {
                                "missed"
                            };

                            // Salva na tabela clínica do paciente
                            let _ = crate::repositories::medicine_repository::MedicineRepository::create_log(
                                pool,
                                user_id,
                                medication_id,
                                situation
                            ).await;
                        }
                    }
                }
            }
        }
    }

    Ok(event)
}

pub async fn process_heartbeat(pool: &PgPool, device_id: &str, user_id: Option<uuid::Uuid>, req: &HeartbeatRequest) -> Result<HeartbeatResponse, ServiceError> {
    let firmware = req.firmware_version.as_deref();

    // Verificar se o schedule foi alterado ANTES de atualizar o heartbeat (só se tiver usuário vinculado)
    let schedule_updated = if let Some(uid) = user_id {
        let last_hb = DeviceRepository::get_last_heartbeat(pool, device_id)
            .await
            .map_err(ServiceError::Database)?;

        DeviceRepository::schedule_updated_since(pool, uid, last_hb)
            .await
            .map_err(ServiceError::Database)?
    } else {
        false
    };

    // Atualizar heartbeat no banco
    DeviceRepository::update_heartbeat(pool, device_id, firmware)
        .await
        .map_err(ServiceError::Database)?;

    Ok(HeartbeatResponse {
        status: "ok".to_string(),
        schedule_updated,
    })
}

pub async fn report_log(pool: &PgPool, device_id: &str, req: &DeviceLogRequest) -> Result<DeviceLog, ServiceError> {
    let event_timestamp = DateTime::from_timestamp(req.timestamp, 0)
        .ok_or(ServiceError::NotFound)?;

    DeviceRepository::insert_log(
        pool,
        device_id,
        &req.level,
        req.component.as_deref(),
        &req.message,
        event_timestamp,
    )
    .await
    .map_err(ServiceError::Database)
}

pub async fn bind_device(pool: &PgPool, user_id: uuid::Uuid, req: &BindDeviceRequest) -> Result<(), ServiceError> {
    let bound = DeviceRepository::bind_to_user(pool, &req.device_id, user_id)
        .await
        .map_err(ServiceError::Database)?;

    if !bound {
        return Err(ServiceError::NotFound); // Dispositivo não existe, já pareado, ou desativado
    }

    Ok(())
}
