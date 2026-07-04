use std::net::SocketAddr;
use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
    Router,
    extract::ConnectInfo,
};
use http_body_util::BodyExt;
use rust_raw_server::{app::{build_app, AppState}, config::Config};
use serde_json::{json, Value};
use serial_test::serial;
use sqlx::PgPool;
use tower::ServiceExt;

async fn setup() -> (Router, PgPool) 
{
    dotenvy::from_filename_override(".env.test").ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env.test");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("failed to connect to database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run migrations");

    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .expect("failed to clean users table");

    let config = Config 
    {
        database_url: database_url.clone(),
        jwt_secret: "chave_secreta_de_testes_super_segura".to_string(),
        rust_log: "info".to_string(),
        app_env: "test".to_string(),
    };

    let state = AppState 
    {
        pool: pool.clone(),
        config,
    };

    let app = build_app(state);

    (app, pool)
}

async fn request
    (
        app: Router,
        method: Method,
        uri: &str,
        body: Option<Value>,
        token: Option<&str>,
    ) -> (StatusCode, Value) 
{
    let mut builder = Request::builder()
        .method(method)
        .uri(uri);

    if body.is_some() 
    {
        builder = builder.header("Content-Type", "application/json");
    }

    if let Some(token) = token 
    {
        builder = builder.header("Authorization", format!("Bearer {}", token));
    }

    let body = match body 
    {
        Some(value) => Body::from(value.to_string()),
        None => Body::empty(),
    };

    let mut req = builder.body(body).unwrap();

    req.extensions_mut().insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 1234))));

    let response = app
        .oneshot(req)
        .await
        .unwrap();

    let status = response.status();

    let bytes = response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();

    if bytes.is_empty() 
    {
        return (status, json!(null));
    }

    let json: Value = serde_json::from_slice(&bytes).unwrap();

    (status, json)
}

async fn register_user(app: Router, email: &str) -> Value {
    let (status, body) = request
    (
        app,
        Method::POST,
        "/auth/register",
        Some
        (
            json!
            (
                {
                    "name": "Stephan",
                    "email": email,
                    "password": "12345678"
                }
            )
        ),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);

    body
}

async fn login_user(app: Router, email: &str) -> String 
{
    let (status, body) = request
    (
        app,
        Method::POST,
        "/auth/login",
        Some
        (
            json!
            (
                {
                    "email": email,
                    "password": "12345678"
                }
            )
        ),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);

    body["access_token"].as_str().unwrap().to_string()
}

#[tokio::test]
#[serial]
async fn health_returns_ok() 
{
    let (app, _) = setup().await;

    let (status, body) = request
    (
        app,
        Method::GET,
        "/health",
        None,
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
#[serial]
async fn register_creates_user() 
{
    let (app, _) = setup().await;

    let body = register_user(app, "stephan@test.com").await;

    assert_eq!(body["id"], 1);
    assert_eq!(body["name"], "Stephan");
    assert_eq!(body["email"], "stephan@test.com");
    assert!(body.get("password_hash").is_none());
}

#[tokio::test]
#[serial]
async fn register_rejects_invalid_payload() 
{
    let (app, _) = setup().await;

    let (status, body) = request
    (
        app,
        Method::POST,
        "/auth/register",
        Some
        (
            json!
            (
                {
                    "name": "A",
                    "email": "email-invalido",
                    "password": "123"
                }
            )
        ),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"], "Invalid register payload");
}

#[tokio::test]
#[serial]
async fn register_rejects_duplicate_email() {
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request
    (
        app,
        Method::POST,
        "/auth/register",
        Some
        (
            json!
            (
                {
                    "name": "Stephan",
                    "email": "stephan@test.com",
                    "password": "12345678"
                }
            )
        ),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"], "User already exists");
}

#[tokio::test]
#[serial]
async fn login_returns_token() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;

    let token = login_user(app, "stephan@test.com").await;

    assert!(!token.is_empty());
}

#[tokio::test]
#[serial]
async fn login_rejects_wrong_password() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request
    (
        app,
        Method::POST,
        "/auth/login",
        Some
        (
            json!
            (
                {
                    "email": "stephan@test.com",
                    "password": "senhaerrada"
                }
            )
        ),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Invalid credentials");
}

#[tokio::test]
#[serial]
async fn list_users_returns_registered_users() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;
    let token = login_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request
    (
        app,
        Method::GET,
        "/users",
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 1);
    assert_eq!(body[0]["email"], "stephan@test.com");
}

#[tokio::test]
#[serial]
async fn get_user_returns_user_by_id() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;
    let token = login_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request
    (
        app,
        Method::GET,
        "/users/1",
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], 1);
    assert_eq!(body["email"], "stephan@test.com");
}

#[tokio::test]
#[serial]
async fn update_user_requires_token() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request
    (
        app,
        Method::PUT,
        "/users/1",
        Some
        (
            json!
            (
                {
                    "name": "Novo Nome"
                }
            )
        ),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Missing or invalid Authorization header");
}

#[tokio::test]
#[serial]
async fn update_user_updates_own_account() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;
    let token = login_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request
    (
        app,
        Method::PUT,
        "/users/1",
        Some(json!({
            "name": "Stephan Atualizado"
        })),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], "Stephan Atualizado");
}

#[tokio::test]
#[serial]
async fn update_user_rejects_other_account() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "user1@test.com").await;
    register_user(app.clone(), "user2@test.com").await;

    let token = login_user(app.clone(), "user1@test.com").await;

    let (status, body) = request
    (
        app,
        Method::PUT,
        "/users/2",
        Some
        (
            json!
            (
                {
                    "name": "Tentativa Indevida"
                }
            )
        ),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "You can only update your own account");
}

#[tokio::test]
#[serial]
async fn delete_user_deletes_own_account() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;
    let token = login_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request
    (
        app.clone(),
        Method::DELETE,
        "/users/1",
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::NO_CONTENT);
    assert_eq!(body, json!(null));

    let (status, body) = request
    (
        app,
        Method::GET,
        "/users/1",
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body["error"], "User not found");
}

#[tokio::test]
#[serial]
async fn delete_user_rejects_other_account() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "user1@test.com").await;
    register_user(app.clone(), "user2@test.com").await;

    let token = login_user(app.clone(), "user1@test.com").await;

    let (status, body) = request(
        app,
        Method::DELETE,
        "/users/2",
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "You can only delete your own account");
}
