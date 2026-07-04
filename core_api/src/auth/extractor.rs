use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use crate::auth::jwt::decode_jwt;
use crate::app::AppState;
use crate::responses::api_response::
{
    ApiError,
    internal_error,
    unauthorized,
};

pub struct AuthUser
{
    pub user_id: i32,
}

impl FromRequestParts<AppState> for AuthUser
{
    type Rejection = ApiError;

    async fn from_request_parts
        (
            parts: &mut Parts,
            state: &AppState,
        ) -> Result<Self, Self::Rejection>
    {
        let auth_header = parts.headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let token = match auth_header
        {
            Some(value) if value.starts_with("Bearer ") =>
            {
                value.trim_start_matches("Bearer ").to_string()
            }
            _ =>
            {
                return Err(unauthorized("Missing or invalid Authorization header"));
            }
        };

        let claims = decode_jwt(&token, &state.config.jwt_secret)
            .map_err(|_| unauthorized("Invalid token"))?;

        let user_id = claims.sub
            .parse::<i32>()
            .map_err(|_| internal_error("Invalid token payload"))?;

        Ok(AuthUser { user_id })
    }
}