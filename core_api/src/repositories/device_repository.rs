use chrono::{DateTime, Utc};
use sqlx::PgPool;

use crate::models::device::{DeviceEvent, DeviceLog, ScheduleEntry};

pub struct DeviceRepository;

impl DeviceRepository {
    /// Buscar a agenda de medicamentos do usuário vinculado ao dispositivo
    pub async fn get_schedule(pool: &PgPool, user_id: uuid::Uuid) -> Result<Vec<ScheduleEntry>, sqlx::Error> {
        let rows = sqlx::query_as!(
            ScheduleEntry,
            r#"
            SELECT id AS medication_id, name, dosage, scheduled_time AS time, compartment, week_days
            FROM medicines
            WHERE user_id = $1
            ORDER BY scheduled_time ASC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await?;

        Ok(rows)
    }

    /// Inserir um evento de telemetria
    pub async fn insert_event(
        pool: &PgPool,
        device_id: &str,
        event_type: &str,
        event_timestamp: DateTime<Utc>,
        metadata: Option<serde_json::Value>,
    ) -> Result<DeviceEvent, sqlx::Error> {
        sqlx::query_as!(
            DeviceEvent,
            r#"
            INSERT INTO device_events (device_id, event_type, event_timestamp, metadata)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            device_id,
            event_type,
            event_timestamp,
            metadata
        )
        .fetch_one(pool)
        .await
    }

    /// Atualizar heartbeat do dispositivo
    pub async fn update_heartbeat(
        pool: &PgPool,
        device_id: &str,
        firmware_version: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE devices
            SET last_heartbeat_at = now(), firmware_version = COALESCE($2, firmware_version)
            WHERE id = $1
            "#,
            device_id,
            firmware_version
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Verificar se o schedule foi alterado desde o último heartbeat
    pub async fn schedule_updated_since(
        pool: &PgPool,
        user_id: uuid::Uuid,
        since: Option<DateTime<Utc>>,
    ) -> Result<bool, sqlx::Error> {
        // Se nunca fez heartbeat, considera como atualizado
        let since = match since {
            Some(ts) => ts,
            None => return Ok(true),
        };

        let row = sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM medicines
                WHERE user_id = $1 AND updated_at > $2
            ) AS "exists!"
            "#,
            user_id,
            since
        )
        .fetch_one(pool)
        .await?;

        Ok(row.exists)
    }

    /// Buscar o último heartbeat do dispositivo
    pub async fn get_last_heartbeat(pool: &PgPool, device_id: &str) -> Result<Option<DateTime<Utc>>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT last_heartbeat_at FROM devices WHERE id = $1
            "#,
            device_id
        )
        .fetch_one(pool)
        .await?;

        Ok(row.last_heartbeat_at)
    }

    /// Inserir log de debug do firmware
    pub async fn insert_log(
        pool: &PgPool,
        device_id: &str,
        level: &str,
        component: Option<&str>,
        message: &str,
        event_timestamp: DateTime<Utc>,
    ) -> Result<DeviceLog, sqlx::Error> {
        sqlx::query_as!(
            DeviceLog,
            r#"
            INSERT INTO device_logs (device_id, level, component, message, event_timestamp)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
            device_id,
            level,
            component,
            message,
            event_timestamp
        )
        .fetch_one(pool)
        .await
    }

    /// Parear dispositivo com usuário (bind)
    pub async fn bind_to_user(pool: &PgPool, device_id: &str, user_id: uuid::Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            UPDATE devices
            SET user_id = $2
            WHERE id = $1 AND user_id IS NULL AND is_active = true
            "#,
            device_id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Buscar o dono atual do dispositivo
    pub async fn get_device_owner(pool: &PgPool, device_id: &str) -> Result<Option<uuid::Uuid>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT user_id FROM devices WHERE id = $1
            "#,
            device_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(row.and_then(|r| r.user_id))
    }
}
