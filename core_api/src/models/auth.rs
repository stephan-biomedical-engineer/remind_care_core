use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterRequest 
{
    #[validate(length(min = 2, max = 100))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String, // Senha em texto puro (só existe na memória)
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest 
{
    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RefreshRequest
{
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LogoutRequest
{
    #[validate(length(min = 1))]
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse 
{
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String, // Geralmente "Bearer"
}

#[derive(Debug, Serialize)]
pub struct AccessTokenResponse
{
    pub access_token: String,
    pub token_type: String,
}
