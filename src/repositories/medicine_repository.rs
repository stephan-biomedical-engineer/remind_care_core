use sqlx::PgPool;
use crate::models::medicine::{Medicine, CreateMedicineRequest, UpdateMedicineRequest, MedicineLog};

pub struct MedicineRepository;

impl MedicineRepository {
    pub async fn create(pool: &PgPool, user_id: i32, req: &CreateMedicineRequest) -> Result<Medicine, sqlx::Error> {
        sqlx::query_as!(
            Medicine,
            r#"
            INSERT INTO medicines (user_id, name, dosage, compartment, scheduled_time, week_days, notes)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *
            "#,
            user_id,
            req.name,
            req.dosage,
            req.compartment,
            req.scheduled_time,
            &req.week_days,
            req.notes
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_user(pool: &PgPool, user_id: i32) -> Result<Vec<Medicine>, sqlx::Error> {
        sqlx::query_as!(
            Medicine,
            r#"
            SELECT * FROM medicines WHERE user_id = $1 ORDER BY scheduled_time ASC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id_and_user(pool: &PgPool, id: i32, user_id: i32) -> Result<Option<Medicine>, sqlx::Error> {
        sqlx::query_as!(
            Medicine,
            r#"
            SELECT * FROM medicines WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn update(pool: &PgPool, id: i32, user_id: i32, req: &UpdateMedicineRequest) -> Result<Option<Medicine>, sqlx::Error> {
        sqlx::query_as!(
            Medicine,
            r#"
            UPDATE medicines
            SET name = $1, dosage = $2, compartment = $3, scheduled_time = $4, week_days = $5, notes = $6, updated_at = CURRENT_TIMESTAMP
            WHERE id = $7 AND user_id = $8
            RETURNING *
            "#,
            req.name,
            req.dosage,
            req.compartment,
            req.scheduled_time,
            &req.week_days,
            req.notes,
            id,
            user_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn delete(pool: &PgPool, id: i32, user_id: i32) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM medicines WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
    
    pub async fn create_log(pool: &PgPool, user_id: i32, medicine_id: i32, situation: &str) -> Result<MedicineLog, sqlx::Error> {
        sqlx::query_as!(
            MedicineLog,
            r#"
            INSERT INTO medicine_logs (user_id, medicine_id, situation, opened_at)
            VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
            RETURNING *
            "#,
            user_id,
            medicine_id,
            situation
        )
        .fetch_one(pool)
        .await
    }
    
    pub async fn get_logs_today(pool: &PgPool, user_id: i32) -> Result<Vec<MedicineLog>, sqlx::Error> {
        sqlx::query_as!(
            MedicineLog,
            r#"
            SELECT * FROM medicine_logs 
            WHERE user_id = $1 AND opened_at >= CURRENT_DATE
            ORDER BY opened_at ASC
            "#,
            user_id
        )
        .fetch_all(pool)
        .await
    }
}
