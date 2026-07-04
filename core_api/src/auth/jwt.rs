use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims 
{
    pub sub: String, // ID do usuário (subject)
    pub exp: usize,  // Data de expiração
}

pub fn hash_password(password: &str) 
    -> Result<String, argon2::password_hash::Error> 
{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) 
    -> bool 
{
    let parsed_hash = match PasswordHash::new(password_hash) 
    {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

pub fn create_jwt
    (
        user_id: i32,
        secret: &str,
    ) -> Result<String, jsonwebtoken::errors::Error>
{
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .expect("Timestamp inválido")
        .timestamp() as usize;

    let claims = Claims 
    {
        sub: user_id.to_string(),
        exp: expiration,
    };

    encode
    (
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn decode_jwt
    (
        token: &str,
        secret: &str,
    ) -> Result<Claims, jsonwebtoken::errors::Error>
{
    let data = decode::<Claims>
    (
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(data.claims)
}

pub fn generate_refresh_token() -> String
{
    Uuid::new_v4().to_string()
}

pub fn hash_refresh_token(token: &str) -> String
{
    let mut hasher = Sha256::new();
    hasher.update(token.as_bytes());
    hex::encode(hasher.finalize())
}
