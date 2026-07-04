use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::auth::jwt::{
    create_jwt,
    generate_refresh_token,
    hash_password,
    hash_refresh_token,
    verify_password,
};
use crate::models::auth::{
    AccessTokenResponse,
    AuthResponse,
    LoginRequest,
    LogoutRequest,
    RefreshRequest,
    RegisterRequest,
};
use crate::models::user::User;
use crate::repositories::{refresh_tokens_repository, users_repository};

pub enum AuthError 
{
    Database(sqlx::Error),
    InvalidCredentials,
    UserAlreadyExists,
    HashingError,
    TokenCreationError,
}

fn is_unique_violation(err: &sqlx::Error) -> bool
{
    match err
    {
        sqlx::Error::Database(db_err) => db_err.code().as_deref() == Some("23505"),
        _ => false,
    }
}

pub async fn register(pool: &PgPool, req: RegisterRequest) -> Result<User, AuthError> 
{
    let password = req.password.clone();

    let hash = tokio::task::spawn_blocking(move || 
    {
        hash_password(&password)
    })
        .await
        .map_err(|_| AuthError::HashingError)?
        .map_err(|_| AuthError::HashingError)?;

    let user = users_repository::create(pool, req.name, req.email, hash)
        .await
        .map_err(|err|
        {
            if is_unique_violation(&err)
            {
                AuthError::UserAlreadyExists
            }
            else
            {
                AuthError::Database(err)
            }
        })?;

    Ok(user)
}

pub async fn login
    (
        pool: &PgPool,
        jwt_secret: &str,
        req: LoginRequest,
    ) -> Result<AuthResponse, AuthError> 
{
    let user = users_repository::find_by_email(pool, &req.email)
        .await
        .map_err(AuthError::Database)?
        .ok_or(AuthError::InvalidCredentials)?; 

    let password = req.password.clone();
    let password_hash = user.password_hash.clone();

    let is_valid = tokio::task::spawn_blocking(move || 
    {
        verify_password(&password, &password_hash)
    })
        .await
        .map_err(|_| AuthError::HashingError)?;

    if !is_valid 
    {
        return Err(AuthError::InvalidCredentials); 
    }

    let access_token = create_jwt(user.id, jwt_secret)
        .map_err(|_| AuthError::TokenCreationError)?;

    let refresh_token = generate_refresh_token();
    let refresh_token_hash = hash_refresh_token(&refresh_token);
    let expires_at = Utc::now() + Duration::days(7);

    refresh_tokens_repository::create(
        pool,
        user.id,
        refresh_token_hash,
        expires_at,
    )
    .await
    .map_err(AuthError::Database)?;

    Ok(AuthResponse 
    {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
    })
}

pub async fn refresh
    (
        pool: &PgPool,
        jwt_secret: &str,
        req: RefreshRequest,
    ) -> Result<AccessTokenResponse, AuthError>
{
    let refresh_token_hash = hash_refresh_token(&req.refresh_token);

    let user_id = refresh_tokens_repository::find_valid_user_id(pool, &refresh_token_hash)
        .await
        .map_err(AuthError::Database)?
        .ok_or(AuthError::InvalidCredentials)?;

    let access_token = create_jwt(user_id, jwt_secret)
        .map_err(|_| AuthError::TokenCreationError)?;

    Ok(AccessTokenResponse
    {
        access_token,
        token_type: "Bearer".to_string(),
    })
}

pub async fn logout
    (
        pool: &PgPool,
        req: LogoutRequest,
    ) -> Result<(), AuthError>
{
    let refresh_token_hash = hash_refresh_token(&req.refresh_token);

    refresh_tokens_repository::revoke(pool, &refresh_token_hash)
        .await
        .map_err(AuthError::Database)?;

    Ok(())
}
