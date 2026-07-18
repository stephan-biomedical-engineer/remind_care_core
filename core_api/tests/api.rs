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
        .expect("failed to clean database tables");

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
        fcm_manager: None,
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

/// Helper to send a request with a custom header (for admin routes)
async fn request_with_header(
    app: Router,
    method: Method,
    uri: &str,
    body: Option<Value>,
    header_name: &str,
    header_value: &str,
) -> (StatusCode, Value) 
{
    let mut builder = Request::builder()
        .method(method)
        .uri(uri)
        .header(header_name, header_value);

    if body.is_some() 
    {
        builder = builder.header("Content-Type", "application/json");
    }

    let body = match body 
    {
        Some(value) => Body::from(value.to_string()),
        None => Body::empty(),
    };

    let mut req = builder.body(body).unwrap();
    req.extensions_mut().insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 1234))));

    let response = app.oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();

    if bytes.is_empty() 
    {
        return (status, json!(null));
    }

    let json: Value = serde_json::from_slice(&bytes).unwrap();
    (status, json)
}

async fn register_user(app: Router, email: &str) -> Value {
    let (status, body) = request(
        app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "name": "Stephan",
            "email": email,
            "password": "12345678"
        })),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::CREATED, "Register failed: {:?}", body);
    body
}

async fn login_user(app: Router, email: &str) -> (String, String) 
{
    let (status, body) = request(
        app,
        Method::POST,
        "/auth/login",
        Some(json!({
            "email": email,
            "password": "12345678"
        })),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK, "Login failed: {:?}", body);

    let access_token = body["access_token"].as_str().unwrap().to_string();
    let refresh_token = body["refresh_token"].as_str().unwrap().to_string();
    (access_token, refresh_token)
}

/// Register + login helper that returns (user_id_string, access_token, refresh_token)
async fn register_and_login(app: Router, email: &str) -> (String, String, String) {
    let reg_body = register_user(app.clone(), email).await;
    let user_id = reg_body["id"].as_str().unwrap().to_string();
    let (access_token, refresh_token) = login_user(app, email).await;
    (user_id, access_token, refresh_token)
}

// ─────────────────────────────────────────────────────────────
//  Health
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn health_returns_ok() 
{
    let (app, _) = setup().await;

    let (status, body) = request(
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

// ─────────────────────────────────────────────────────────────
//  Auth: Register
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn register_creates_user() 
{
    let (app, _) = setup().await;

    let body = register_user(app, "stephan@test.com").await;

    // ID is now a UUID string
    assert!(body["id"].as_str().is_some());
    assert_eq!(body["name"], "Stephan");
    assert_eq!(body["email"], "stephan@test.com");
    assert!(body.get("password_hash").is_none());
}

#[tokio::test]
#[serial]
async fn register_rejects_invalid_payload() 
{
    let (app, _) = setup().await;

    let (status, body) = request(
        app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "name": "A",
            "email": "email-invalido",
            "password": "123"
        })),
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

    let (status, body) = request(
        app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "name": "Stephan",
            "email": "stephan@test.com",
            "password": "12345678"
        })),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body["error"], "User already exists");
}

// ─────────────────────────────────────────────────────────────
//  Auth: Login
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn login_returns_tokens() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;

    let (access_token, refresh_token) = login_user(app, "stephan@test.com").await;

    assert!(!access_token.is_empty());
    assert!(!refresh_token.is_empty());
}

#[tokio::test]
#[serial]
async fn login_rejects_wrong_password() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request(
        app,
        Method::POST,
        "/auth/login",
        Some(json!({
            "email": "stephan@test.com",
            "password": "senhaerrada"
        })),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Invalid credentials");
}

#[tokio::test]
#[serial]
async fn login_rejects_nonexistent_user() 
{
    let (app, _) = setup().await;

    let (status, body) = request(
        app,
        Method::POST,
        "/auth/login",
        Some(json!({
            "email": "fantasma@test.com",
            "password": "12345678"
        })),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Invalid credentials");
}

// ─────────────────────────────────────────────────────────────
//  Auth: Refresh Token
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn refresh_returns_new_access_token() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;
    let (_access_token, refresh_token) = login_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request(
        app,
        Method::POST,
        "/auth/refresh",
        Some(json!({
            "refresh_token": refresh_token
        })),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["access_token"].as_str().is_some());
    assert_eq!(body["token_type"], "Bearer");
}

#[tokio::test]
#[serial]
async fn refresh_rejects_invalid_token() 
{
    let (app, _) = setup().await;

    let (status, body) = request(
        app,
        Method::POST,
        "/auth/refresh",
        Some(json!({
            "refresh_token": "token-invalido-que-nao-existe"
        })),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Invalid credentials");
}

// ─────────────────────────────────────────────────────────────
//  Auth: Logout
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn logout_revokes_refresh_token() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;
    let (_access_token, refresh_token) = login_user(app.clone(), "stephan@test.com").await;

    // Logout (revoga o refresh token)
    let (status, _) = request(
        app.clone(),
        Method::POST,
        "/auth/logout",
        Some(json!({
            "refresh_token": refresh_token.clone()
        })),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::NO_CONTENT);

    // Tentar usar o refresh token revogado deve falhar
    let (status, body) = request(
        app,
        Method::POST,
        "/auth/refresh",
        Some(json!({
            "refresh_token": refresh_token
        })),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Invalid credentials");
}

// ─────────────────────────────────────────────────────────────
//  Users: List
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn list_users_returns_registered_users() 
{
    let (app, _) = setup().await;

    register_user(app.clone(), "stephan@test.com").await;
    let (token, _) = login_user(app.clone(), "stephan@test.com").await;

    let (status, body) = request(
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
async fn list_users_rejects_without_token() 
{
    let (app, _) = setup().await;

    let (status, body) = request(
        app,
        Method::GET,
        "/users",
        None,
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Missing or invalid Authorization header");
}

// ─────────────────────────────────────────────────────────────
//  Users: Get by ID
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn get_user_returns_own_profile() 
{
    let (app, _) = setup().await;

    let (user_id, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    let (status, body) = request(
        app,
        Method::GET,
        &format!("/users/{}", user_id),
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], user_id);
    assert_eq!(body["email"], "stephan@test.com");
}

#[tokio::test]
#[serial]
async fn get_user_rejects_other_profile() 
{
    let (app, _) = setup().await;

    let (_user1_id, token1, _) = register_and_login(app.clone(), "user1@test.com").await;
    let (user2_id, _, _) = register_and_login(app.clone(), "user2@test.com").await;

    let (status, body) = request(
        app,
        Method::GET,
        &format!("/users/{}", user2_id),
        None,
        Some(&token1),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "You can only view your own account");
}

// ─────────────────────────────────────────────────────────────
//  Users: Update
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn update_user_requires_token() 
{
    let (app, _) = setup().await;

    let body = register_user(app.clone(), "stephan@test.com").await;
    let user_id = body["id"].as_str().unwrap();

    let (status, body) = request(
        app,
        Method::PUT,
        &format!("/users/{}", user_id),
        Some(json!({
            "name": "Novo Nome"
        })),
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

    let (user_id, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    let (status, body) = request(
        app,
        Method::PUT,
        &format!("/users/{}", user_id),
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

    let (_user1_id, token1, _) = register_and_login(app.clone(), "user1@test.com").await;
    let (user2_id, _, _) = register_and_login(app.clone(), "user2@test.com").await;

    let (status, body) = request(
        app,
        Method::PUT,
        &format!("/users/{}", user2_id),
        Some(json!({
            "name": "Tentativa Indevida"
        })),
        Some(&token1),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "You can only update your own account");
}

// ─────────────────────────────────────────────────────────────
//  Users: Delete
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn delete_user_deletes_own_account() 
{
    let (app, _) = setup().await;

    let (user_id, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    let (status, _) = request(
        app.clone(),
        Method::DELETE,
        &format!("/users/{}", user_id),
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::NO_CONTENT);

    // Confirmar que o usuário foi de fato excluído
    let (status, body) = request(
        app,
        Method::GET,
        &format!("/users/{}", user_id),
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

    let (_user1_id, token1, _) = register_and_login(app.clone(), "user1@test.com").await;
    let (user2_id, _, _) = register_and_login(app.clone(), "user2@test.com").await;

    let (status, body) = request(
        app,
        Method::DELETE,
        &format!("/users/{}", user2_id),
        None,
        Some(&token1),
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "You can only delete your own account");
}

// ─────────────────────────────────────────────────────────────
//  Medicines: CRUD
// ─────────────────────────────────────────────────────────────

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

#[tokio::test]
#[serial]
async fn create_medicine_success() 
{
    let (app, _) = setup().await;
    let (_, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    let (status, body) = request(
        app,
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED, "Failed to create medicine: {:?}", body);
    assert!(body["id"].as_str().is_some()); // UUID string
    assert_eq!(body["name"], "Paracetamol");
    assert_eq!(body["dosage"], "500mg");
    assert_eq!(body["compartment"], 1);
}

#[tokio::test]
#[serial]
async fn create_medicine_rejects_without_token() 
{
    let (app, _) = setup().await;

    let (status, body) = request(
        app,
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Missing or invalid Authorization header");
}

#[tokio::test]
#[serial]
async fn create_medicine_rejects_invalid_payload() 
{
    let (app, _) = setup().await;
    let (_, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    let (status, body) = request(
        app,
        Method::POST,
        "/medicines",
        Some(json!({
            "name": "",
            "dosage": "",
            "compartment": 1,
            "scheduled_time": "14:00:00",
            "week_days": [1]
        })),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert_eq!(body["error"], "Invalid medicine payload");
}

#[tokio::test]
#[serial]
async fn list_medicines_returns_user_medicines() 
{
    let (app, _) = setup().await;
    let (_, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    // Criar 2 medicamentos
    request(app.clone(), Method::POST, "/medicines", Some(medicine_payload()), Some(&token)).await;
    request(app.clone(), Method::POST, "/medicines", Some(json!({
        "name": "Ibuprofeno",
        "dosage": "200mg",
        "compartment": 2,
        "scheduled_time": "08:00:00",
        "week_days": [0, 1, 2, 3, 4, 5, 6],
        "notes": null
    })), Some(&token)).await;

    let (status, body) = request(
        app,
        Method::GET,
        "/medicines",
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 2);
}

#[tokio::test]
#[serial]
async fn get_medicine_by_id() 
{
    let (app, _) = setup().await;
    let (_, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    let (_, created) = request(
        app.clone(),
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        Some(&token),
    )
    .await;

    let med_id = created["id"].as_str().unwrap();

    let (status, body) = request(
        app,
        Method::GET,
        &format!("/medicines/{}", med_id),
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"], med_id);
    assert_eq!(body["name"], "Paracetamol");
}

#[tokio::test]
#[serial]
async fn update_medicine_success() 
{
    let (app, _) = setup().await;
    let (_, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    let (_, created) = request(
        app.clone(),
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        Some(&token),
    )
    .await;

    let med_id = created["id"].as_str().unwrap();

    let (status, body) = request(
        app,
        Method::PUT,
        &format!("/medicines/{}", med_id),
        Some(json!({
            "name": "Paracetamol Forte",
            "dosage": "750mg",
            "compartment": 3,
            "scheduled_time": "20:00:00",
            "week_days": [1, 2, 3, 4, 5],
            "notes": "Dose noturna"
        })),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"], "Paracetamol Forte");
    assert_eq!(body["dosage"], "750mg");
    assert_eq!(body["compartment"], 3);
}

#[tokio::test]
#[serial]
async fn delete_medicine_success() 
{
    let (app, _) = setup().await;
    let (_, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    let (_, created) = request(
        app.clone(),
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        Some(&token),
    )
    .await;

    let med_id = created["id"].as_str().unwrap();

    let (status, _) = request(
        app.clone(),
        Method::DELETE,
        &format!("/medicines/{}", med_id),
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::NO_CONTENT);

    // Confirmar que não existe mais
    let (status, _) = request(
        app,
        Method::GET,
        &format!("/medicines/{}", med_id),
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
#[serial]
async fn medicine_isolation_between_users() 
{
    let (app, _) = setup().await;

    let (_, token1, _) = register_and_login(app.clone(), "user1@test.com").await;
    let (_, token2, _) = register_and_login(app.clone(), "user2@test.com").await;

    // User 1 cria um medicamento
    let (_, created) = request(
        app.clone(),
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        Some(&token1),
    )
    .await;

    let med_id = created["id"].as_str().unwrap();

    // User 2 tenta acessar o medicamento do user 1 → 404
    let (status, _) = request(
        app.clone(),
        Method::GET,
        &format!("/medicines/{}", med_id),
        None,
        Some(&token2),
    )
    .await;

    assert_eq!(status, StatusCode::NOT_FOUND);

    // User 2 lista seus medicamentos → vazio
    let (status, body) = request(
        app,
        Method::GET,
        "/medicines",
        None,
        Some(&token2),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 0);
}

// ─────────────────────────────────────────────────────────────
//  Medicines: Logs
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn create_medicine_log_success() 
{
    let (app, _) = setup().await;
    let (_, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    // Criar medicamento primeiro
    let (_, created) = request(
        app.clone(),
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        Some(&token),
    )
    .await;

    let med_id = created["id"].as_str().unwrap();

    // Registrar abertura da caixa
    let (status, body) = request(
        app,
        Method::POST,
        "/medicines/logs",
        Some(json!({
            "medicine_id": med_id,
            "situation": "onTime"
        })),
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["id"].as_str().is_some());
    assert_eq!(body["medicine_id"], med_id);
    assert_eq!(body["situation"], "onTime");
}

#[tokio::test]
#[serial]
async fn get_today_logs_returns_logs() 
{
    let (app, _) = setup().await;
    let (_, token, _) = register_and_login(app.clone(), "stephan@test.com").await;

    // Criar medicamento
    let (_, created) = request(
        app.clone(),
        Method::POST,
        "/medicines",
        Some(medicine_payload()),
        Some(&token),
    )
    .await;

    let med_id = created["id"].as_str().unwrap();

    // Registrar 2 logs
    request(app.clone(), Method::POST, "/medicines/logs", Some(json!({
        "medicine_id": med_id,
        "situation": "onTime"
    })), Some(&token)).await;

    request(app.clone(), Method::POST, "/medicines/logs", Some(json!({
        "medicine_id": med_id,
        "situation": "late"
    })), Some(&token)).await;

    // Buscar logs do dia
    let (status, body) = request(
        app,
        Method::GET,
        "/medicines/logs",
        None,
        Some(&token),
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 2);
}

// ─────────────────────────────────────────────────────────────
//  Admin: Provision
// ─────────────────────────────────────────────────────────────

#[tokio::test]
#[serial]
async fn admin_provision_creates_device() 
{
    let (app, _) = setup().await;

    let (status, body) = request_with_header(
        app,
        Method::POST,
        "/api/v1/admin/provision",
        None,
        "X-Admin-Secret",
        "test_admin_secret",
    )
    .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["device_id"].as_str().unwrap().starts_with("RC-"));
    assert!(body["api_key"].as_str().is_some());
    assert!(body["api_key"].as_str().unwrap().len() >= 40);
}

#[tokio::test]
#[serial]
async fn admin_provision_rejects_wrong_secret() 
{
    let (app, _) = setup().await;

    let (status, body) = request_with_header(
        app,
        Method::POST,
        "/api/v1/admin/provision",
        None,
        "X-Admin-Secret",
        "wrong_secret",
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Unauthorized: Invalid admin secret key");
}

#[tokio::test]
#[serial]
async fn admin_provision_rejects_missing_secret() 
{
    let (app, _) = setup().await;

    let (status, body) = request(
        app,
        Method::POST,
        "/api/v1/admin/provision",
        None,
        None,
    )
    .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(body["error"], "Unauthorized: Invalid admin secret key");
}
