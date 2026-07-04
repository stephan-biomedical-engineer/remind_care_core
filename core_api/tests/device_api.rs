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

// ─────────────────────────────────────────────────────────────
//  Setup & Helpers
// ─────────────────────────────────────────────────────────────

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

    sqlx::query("TRUNCATE TABLE users, refresh_tokens, medicines, medicine_logs, devices, device_events, device_logs RESTART IDENTITY CASCADE")
        .execute(&pool)
        .await
        .expect("failed to clean tables");

    let config = Config 
    {
        database_url: database_url.clone(),
        jwt_secret: "chave_secreta_de_testes_super_segura".to_string(),
        admin_secret_key: "test_admin_secret".to_string(),
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

async fn request(
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

async fn register_and_login(app: Router, email: &str) -> (String, String) {
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
    
    let user_id = body["id"].as_str().unwrap().to_string();

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

fn medicine_payload() -> Value {
    json!({
        "name": "Paracetamol",
        "dosage": "500mg",
        "compartment": 1,
        "scheduled_time": "14:00:00",
        "week_days": [1, 3, 5],
        "notes": "Tomar após o almoço"
    })
}

// ─────────────────────────────────────────────────────────────
//  Device: Heartbeat
// ─────────────────────────────────────────────────────────────

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
    assert_eq!(body["schedule_updated"], false);
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
async fn device_rejects_missing_auth() 
{
    let (app, pool) = setup().await;
    seed_device(&pool, "PI-001", "secret_device_key_123").await;

    let (status, _body) = request(
        app,
        Method::POST,
        "/api/v1/devices/heartbeat",
        Some(json!({
            "uptime_seconds": 3600
        })),
        None,
    ).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// ─────────────────────────────────────────────────────────────
//  Device: Bind + Schedule
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn bind_device_to_user_and_get_schedule() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-002";
    let api_key = "secret_key_456";
    seed_device(&pool, device_id, api_key).await;

    // 1. App mobile pareia a caixa
    let (_user_id, jwt) = register_and_login(app.clone(), "patient@test.com").await;

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
async fn schedule_returns_medicines_after_bind() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-004";
    let api_key = "secret_key_schedule";
    seed_device(&pool, device_id, api_key).await;

    // Registrar, logar, e criar medicamento
    let (_user_id, jwt) = register_and_login(app.clone(), "patient2@test.com").await;

    request(
        app.clone(),
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        Some(&jwt),
    ).await;

    // Parear dispositivo
    request(
        app.clone(),
        Method::POST,
        "/api/v1/devices/bind",
        Some(json!({ "device_id": device_id })),
        Some(&jwt),
    ).await;

    // Caixa pede schedule → deve ter 1 medicamento
    let (status, body) = request(
        app,
        Method::GET,
        "/api/v1/devices/schedule",
        None,
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["schedule"].as_array().unwrap().len(), 1);
    assert_eq!(body["schedule"][0]["name"], "Paracetamol");
    assert_eq!(body["schedule"][0]["dosage"], "500mg");
}

#[tokio::test]
#[serial]
async fn schedule_returns_not_found_for_unbound_device() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-SOLO";
    let api_key = "secret_key_solo";
    seed_device(&pool, device_id, api_key).await;

    // Dispositivo sem pareamento tenta pedir schedule
    let (status, _) = request(
        app,
        Method::GET,
        "/api/v1/devices/schedule",
        None,
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// ─────────────────────────────────────────────────────────────
//  Device: Events
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn device_event_success() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-003";
    let api_key = "secret_key_789";
    seed_device(&pool, device_id, api_key).await;

    let (status, body) = request(
        app,
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
}

#[tokio::test]
#[serial]
async fn device_event_box_closed() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-005";
    let api_key = "secret_key_close";
    seed_device(&pool, device_id, api_key).await;

    let (status, body) = request(
        app,
        Method::POST,
        "/api/v1/devices/events",
        Some(json!({
            "event_type": "box_closed",
            "timestamp": 1719803800,
            "metadata": { "compartment_opened": 2 }
        })),
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["event_type"], "box_closed");
}

// ─────────────────────────────────────────────────────────────
//  Device: Logs
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn device_log_success() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-006";
    let api_key = "secret_key_log";
    seed_device(&pool, device_id, api_key).await;

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

#[tokio::test]
#[serial]
async fn device_log_warn_level() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-007";
    let api_key = "secret_key_warn";
    seed_device(&pool, device_id, api_key).await;

    let (status, body) = request(
        app,
        Method::POST,
        "/api/v1/devices/logs",
        Some(json!({
            "level": "WARN",
            "component": "battery_monitor",
            "message": "Battery below 20%",
            "timestamp": 1719803900
        })),
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["level"], "WARN");
    assert_eq!(body["component"], "battery_monitor");
}

// ─────────────────────────────────────────────────────────────
//  Device: Heartbeat detects schedule changes
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn heartbeat_detects_schedule_update() 
{
    let (app, pool) = setup().await;
    let device_id = "PI-008";
    let api_key = "secret_key_hb_update";
    seed_device(&pool, device_id, api_key).await;

    // Parear e criar medicamento
    let (_user_id, jwt) = register_and_login(app.clone(), "patient3@test.com").await;
    
    request(
        app.clone(),
        Method::POST,
        "/api/v1/devices/bind",
        Some(json!({ "device_id": device_id })),
        Some(&jwt),
    ).await;

    // Primeiro heartbeat (nenhum heartbeat anterior → schedule_updated = true)
    let (status, body) = request(
        app.clone(),
        Method::POST,
        "/api/v1/devices/heartbeat",
        Some(json!({
            "uptime_seconds": 100,
            "firmware_version": "1.0.0"
        })),
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "ok");
    assert_eq!(body["schedule_updated"], true);

    // Segundo heartbeat (nada mudou → schedule_updated = false)
    let (status, body) = request(
        app.clone(),
        Method::POST,
        "/api/v1/devices/heartbeat",
        Some(json!({
            "uptime_seconds": 200,
            "firmware_version": "1.0.0"
        })),
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["schedule_updated"], false);

    // Criar um medicamento (atualiza a tabela medicines com updated_at > last_heartbeat)
    request(
        app.clone(),
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        Some(&jwt),
    ).await;

    // Terceiro heartbeat → schedule_updated = true (medicamento criado depois do último hb)
    let (status, body) = request(
        app,
        Method::POST,
        "/api/v1/devices/heartbeat",
        Some(json!({
            "uptime_seconds": 300,
            "firmware_version": "1.0.0"
        })),
        Some(api_key),
    ).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["schedule_updated"], true);
}
