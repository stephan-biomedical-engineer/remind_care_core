#[derive(Debug, Clone)]
pub struct Config
{
    pub database_url: String,
    pub jwt_secret: String,
    pub admin_secret_key: String,
    pub rust_log: String,
    pub app_env: String,
}

impl Config
{
    pub fn from_env() -> Self
    {
        Self
        {
            database_url: std::env::var("DATABASE_URL")
                .expect("[ERROR] DATABASE_URL must be set"),

            jwt_secret: std::env::var("JWT_SECRET")
                .expect("[ERROR] JWT_SECRET must be set"),

            admin_secret_key: std::env::var("ADMIN_SECRET_KEY")
                .expect("[ERROR] ADMIN_SECRET_KEY must be set"),

            rust_log: std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "info".to_string()),

            app_env: std::env::var("APP_ENV")
                .unwrap_or_else(|_| "development".to_string()),
        }
    }

    pub fn is_production(&self) -> bool
    {
        self.app_env == "production"
    }
}