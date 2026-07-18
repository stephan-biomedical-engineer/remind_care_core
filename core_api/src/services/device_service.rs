use chrono::DateTime;
use sqlx::PgPool;

use crate::models::device::*;
use crate::repositories::device_repository::DeviceRepository;
use crate::services::users_service::ServiceError;
use oauth_fcm::{send_fcm_message, FcmNotification, SharedTokenManager};

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

pub async fn report_event(
    pool: &PgPool, 
    fcm_manager: &Option<SharedTokenManager>, 
    project_id: &str, 
    device_id: &str, 
    req: &DeviceEventRequest
) -> Result<DeviceEvent, ServiceError> {
    let event_timestamp = DateTime::from_timestamp(req.timestamp, 0)
        .ok_or(ServiceError::NotFound)?; // timestamp inválido

    let event = DeviceRepository::insert_event(pool, device_id, &req.event_type, event_timestamp, req.metadata.clone())
        .await
        .map_err(ServiceError::Database)?;    // Tradução de Telemetria (Hardware Edge AI) -> Evento Clínico (Aplicação)
    if req.event_type == "medication_status" {
        if let Some(meta) = &req.metadata {
            if let Ok(Some(user_id)) = DeviceRepository::get_device_owner(pool, device_id).await {
                
                let mut situation = "missed".to_string();
                let mut medication_id_opt: Option<uuid::Uuid> = None;

                if let Some(med_id_val) = meta.get("medication_id") {
                    if let Some(med_id_str) = med_id_val.as_str() {
                        if let Ok(med_id) = uuid::Uuid::parse_str(med_id_str) {
                            medication_id_opt = Some(med_id);
                        }
                    }
                } 
                
                if let Some(sit_val) = meta.get("situation") {
                    if let Some(sit_str) = sit_val.as_str() {
                        situation = sit_str.to_string();
                    }
                }

                // Salva na tabela clínica do paciente de forma agnóstica
                if let Some(medication_id) = medication_id_opt {
                    let _ = crate::repositories::medicine_repository::MedicineRepository::create_log(
                        pool,
                        user_id,
                        medication_id,
                        &situation
                    ).await;
                    
                    // Worker de Notificação Push
                    if let Some(manager) = fcm_manager {
                        if let Ok(Some(user)) = crate::repositories::users_repository::find_by_id(pool, user_id).await {
                            if let Some(token) = user.fcm_token {
                                let title = "Caixa Inteligente".to_string();
                                let body = match situation.as_str() {
                                    "onTime" => "O remédio foi tomado no horário!".to_string(),
                                    "early" => "O remédio foi tomado adiantado.".to_string(),
                                    "late" => "O remédio foi tomado com atraso.".to_string(),
                                    "missed" => "Alerta: O remédio não foi tomado e foi registrado como esquecido!".to_string(),
                                    _ => format!("Status do remédio: {}", situation),
                                };
                                
                                let notification = FcmNotification {
                                    title,
                                    body,
                                };
                                
                                let manager_clone = manager.clone();
                                let project_id = project_id.to_string();
                                
                                tokio::spawn(async move {
                                    let _ = send_fcm_message(&token, Some(notification), None::<serde_json::Value>, &manager_clone, &project_id).await;
                                });
                            }
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

pub async fn get_user_device(pool: &PgPool, user_id: uuid::Uuid) -> Result<Device, ServiceError> {
    DeviceRepository::get_device_by_user(pool, user_id)
        .await
        .map_err(ServiceError::Database)?
        .ok_or(ServiceError::NotFound)
}
