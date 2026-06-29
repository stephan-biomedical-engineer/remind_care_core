use chrono::{NaiveTime, DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Medicine {
    pub id: i32,
    #[serde(skip_serializing)] // Hide from JSON responses
    pub user_id: i32,
    pub name: String,
    pub dosage: String,
    pub compartment: i32,
    pub scheduled_time: NaiveTime,
    pub week_days: Vec<i16>,
    pub notes: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMedicineRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub dosage: String,
    pub compartment: i32,
    pub scheduled_time: NaiveTime, // Formato esperado: "HH:MM:SS"
    pub week_days: Vec<i16>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateMedicineRequest {
    #[validate(length(min = 1, max = 255))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub dosage: String,
    pub compartment: i32,
    pub scheduled_time: NaiveTime,
    pub week_days: Vec<i16>,
    pub notes: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct MedicineLog {
    pub id: i32,
    #[serde(skip_serializing)]
    pub user_id: i32,
    pub medicine_id: i32,
    pub situation: String,
    pub opened_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMedicineLogRequest {
    pub medicine_id: i32,
    #[validate(length(min = 1, max = 50))]
    pub situation: String,
}
