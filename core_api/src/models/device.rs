use chrono::{NaiveTime, DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

// ─── Entidade do Banco ───

#[derive(Debug, sqlx::FromRow)]
pub struct Device {
    pub id: String,
    pub user_id: Option<i32>,
    pub api_key_hash: String,
    pub firmware_version: Option<String>,
    pub last_heartbeat_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct DeviceEvent {
    pub id: i32,
    pub device_id: String,
    pub event_type: String,
    pub event_timestamp: DateTime<Utc>,
    pub metadata: Option<serde_json::Value>,
    pub received_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct DeviceLog {
    pub id: i32,
    pub device_id: String,
    pub level: String,
    pub component: Option<String>,
    pub message: String,
    pub event_timestamp: DateTime<Utc>,
    pub received_at: Option<DateTime<Utc>>,
}

// ─── Requests (Payloads da Caixa) ───

#[derive(Debug, Deserialize, Validate)]
pub struct DeviceEventRequest {
    #[validate(length(min = 1, max = 50))]
    pub event_type: String,
    pub timestamp: i64, // Unix timestamp gerado pela caixa
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct HeartbeatRequest {
    pub uptime_seconds: i64,
    pub network_strength_dbm: Option<i32>,
    #[validate(length(max = 20))]
    pub firmware_version: Option<String>,
    pub unsynced_events: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct DeviceLogRequest {
    #[validate(length(min = 1, max = 10))]
    pub level: String,
    #[validate(length(max = 100))]
    pub component: Option<String>,
    #[validate(length(min = 1))]
    pub message: String,
    pub timestamp: i64,
}

// ─── Requests (Payload do App Mobile) ───

#[derive(Debug, Deserialize, Validate)]
pub struct BindDeviceRequest {
    #[validate(length(min = 1, max = 50))]
    pub device_id: String,
}

// ─── Responses ───

#[derive(Debug, Serialize)]
pub struct ScheduleEntry {
    pub medication_id: i32,
    pub name: String,
    pub dosage: String,
    pub time: NaiveTime,
    pub compartment: i32,
    pub week_days: Vec<i16>,
}

#[derive(Debug, Serialize)]
pub struct ScheduleResponse {
    pub device_id: String,
    pub schedule: Vec<ScheduleEntry>,
}

#[derive(Debug, Serialize)]
pub struct HeartbeatResponse {
    pub status: String,
    pub schedule_updated: bool,
}
