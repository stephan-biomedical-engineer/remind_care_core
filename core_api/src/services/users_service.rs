use sqlx::PgPool;

use crate::models::user::User;
use crate::repositories::users_repository;

pub enum ServiceError
{
    Database(sqlx::Error),
    NotFound,
}

pub async fn list_users(pool: &PgPool) -> Result<Vec<User>, ServiceError>
{
    users_repository::find_all(pool)
        .await
        .map_err(ServiceError::Database)
}

pub async fn get_user(
    pool: &PgPool,
    id: i32,
) -> Result<User, ServiceError>
{
    users_repository::find_by_id(pool, id)
        .await
        .map_err(ServiceError::Database)?
        .ok_or(ServiceError::NotFound)
}

pub async fn update_user
(
    pool: &PgPool,
    id: i32,
    name: String,
) -> Result<User, ServiceError>
{
    users_repository::update(pool, id, name)
        .await
        .map_err(ServiceError::Database)?
        .ok_or(ServiceError::NotFound)
}

pub async fn delete_user(
    pool: &PgPool,
    id: i32,
) -> Result<(), ServiceError>
{
    let rows_affected = users_repository::delete(pool, id)
        .await
        .map_err(ServiceError::Database)?;

    if rows_affected == 0
    {
        return Err(ServiceError::NotFound);
    }

    Ok(())
}