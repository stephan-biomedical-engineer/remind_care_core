use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, sqlx::FromRow)]
pub struct User 
{
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize)]
pub struct PublicUser
{
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
}

impl From<User> for PublicUser
{
    fn from(user: User) -> Self
    {
        Self
        {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest
{
    #[validate(length(min = 2, max = 100))]
    pub name: String,
}