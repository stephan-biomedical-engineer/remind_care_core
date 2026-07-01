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
use sha2::{Digest, Sha256};

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

    sqlx::query("TRUNCATE TABLE users, devices RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .expect("failed to clean tables");

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

async fn seed_device(pool: &PgPool, device_id: &str, api_key: &str) {
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let key_hash = hex::encode(hasher.finalize());

    sqlx::query!(
        "INSERT INTO devices (id, api_key_hash, is_active) VALUES ($1, $2, true)",
        device_id,
        key_hash
    )
    .execute(pool)
    .await
    .unwrap();
}

async fn register_and_login_user(app: Router, email: &str) -> (i32, String) {
    let (_, body) = request(
        app.clone(),
        Method::POST,
        "/auth/register",
        Some(json!({
            "name": "Test User",
            "email": email,
            "password": "password123"
        })),
        None,
    ).await;
    
    let user_id = body["id"].as_i64().unwrap() as i32;

    let (_, body) = request(
        app,
        Method::POST,
        "/auth/login",
        Some(json!({
            "email": email,
            "password": "password123"
        })),
        None,
    ).await;

    let token = body["access_token"].as_str().unwrap().to_string();
    (user_id, token)
}

#[tokio::test]
#[serial]
async fn device_heartbeat_success() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-001";
    let api_key = "secret_device_key_123";
    seed_device(&pool, device_id, api_key).await;

    let (status, body) = request(
        app,
        Method::POST,
        "/api/v1/devices/heartbeat",
        Some(json!({
            "uptime_seconds": 3600,
            "network_strength_dbm": -50,
            "firmware_version": "1.0.0",
            "unsynced_events": 0
        })),
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
    assert_eq!(body["schedule_updated"], false); // No user bound, so false
}

#[tokio::test]
#[serial]
async fn device_rejects_invalid_api_key() 
{
    let (app, pool) = setup().await;
    seed_device(&pool, "PI-001", "secret_device_key_123").await;

    let (status, body) = request(
        app,
        Method::POST,
        "/api/v1/devices/heartbeat",
        Some(json!({
            "uptime_seconds": 3600
        })),
        Some("wrong_key"),
    ).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Unknown device");
}

#[tokio::test]
#[serial]
async fn bind_device_to_user_and_get_schedule() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-002";
    let api_key = "secret_key_456";
    seed_device(&pool, device_id, api_key).await;

    // 1. App mobile pareia a caixa
    let (_user_id, jwt) = register_and_login_user(app.clone(), "patient@test.com").await;

    let (status, _) = request(
        app.clone(),
        Method::POST,
        "/api/v1/devices/bind",
        Some(json!({
            "device_id": device_id
        })),
        Some(&jwt),
    ).await;

    assert_eq!(status, StatusCode::NO_CONTENT);

    // 2. Caixa pede o schedule (deve funcionar e retornar array vazio pois não criamos remédios)
    let (status, body) = request(
        app,
        Method::GET,
        "/api/v1/devices/schedule",
        None,
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["device_id"], device_id);
    assert_eq!(body["schedule"].as_array().unwrap().len(), 0);
}

#[tokio::test]
#[serial]
async fn device_events_and_logs() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-003";
    let api_key = "secret_key_789";
    seed_device(&pool, device_id, api_key).await;

    // Report Event
    let (status, body) = request(
        app.clone(),
        Method::POST,
        "/api/v1/devices/events",
        Some(json!({
            "event_type": "box_opened",
            "timestamp": 1719803763,
            "metadata": { "duration": 5000 }
        })),
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["event_type"], "box_opened");

    // Report Log
    let (status, body) = request(
        app,
        Method::POST,
        "/api/v1/devices/logs",
        Some(json!({
            "level": "ERROR",
            "component": "sensor_hall",
            "message": "Failed to read sensor",
            "timestamp": 1719803770
        })),
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["level"], "ERROR");
    assert_eq!(body["component"], "sensor_hall");
}
