use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create(
    pool: &PgPool,
    user_id: i32,
    token_hash: String,
    expires_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_valid_user_id(
    pool: &PgPool,
    token_hash: &str,
) -> Result<Option<i32>, sqlx::Error> {
    sqlx::query_scalar::<_, i32>(
        "SELECT user_id
         FROM refresh_tokens
         WHERE token_hash = $1
           AND revoked_at IS NULL
           AND expires_at > now()"
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await
}

pub async fn revoke(
    pool: &PgPool,
    token_hash: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE refresh_tokens
         SET revoked_at = now()
         WHERE token_hash = $1
           AND revoked_at IS NULL"
    )
    .bind(token_hash)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}
