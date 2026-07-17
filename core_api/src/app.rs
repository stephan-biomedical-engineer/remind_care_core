use std::sync::Arc;
use tokio::time::{sleep, Duration};
use sqlx::PgPool;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::cors::CorsLayer;
use oauth_fcm::SharedTokenManager;


use axum::
{
    http::
    {
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::{get, post},
    Router,
};

use crate::config::Config;

use crate::routes::
{
    auth::{login, logout, refresh, register},
    health::health,
    users::{delete_user, get_user, list_users, update_user},
};

#[derive(Clone)]
pub struct AppState 
{
    pub pool: PgPool,
    pub config: Config,
    pub fcm_manager: Option<SharedTokenManager>,
}

pub fn build_app(state: AppState) -> Router 
{
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::AllowOrigin::predicate(
            |origin: &axum::http::HeaderValue, _request_parts: &axum::http::request::Parts| {
                let origin_str = origin.to_str().unwrap_or("");
                origin_str == "https://remindcare.com.br"
                    || origin_str.starts_with("http://localhost:")
                    || origin_str.starts_with("http://127.0.0.1:")
            },
        ))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let governor_conf = Arc::new
    (
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(10)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();

    tokio::spawn(async move 
    {
        loop 
        {
            sleep(Duration::from_secs(300)).await;

            tracing::debug!
            (
                "rate limiting storage size: {}",
                governor_limiter.len()
            );

            governor_limiter.retain_recent();
        }
    });

    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh))
        .route("/logout", post(logout))
        .layer(GovernorLayer::new(governor_conf));

    let medicine_routes = Router::new()
        .route("/", get(crate::routes::medicine::list_medicines).post(crate::routes::medicine::create_medicine))
        .route("/logs", get(crate::routes::medicine::get_today_logs).post(crate::routes::medicine::create_log))
        .route("/{id}", get(crate::routes::medicine::get_medicine).put(crate::routes::medicine::update_medicine).delete(crate::routes::medicine::delete_medicine));

    let device_routes = Router::new()
        .route("/schedule", get(crate::routes::device::get_schedule))
        .route("/events", post(crate::routes::device::report_event))
        .route("/heartbeat", post(crate::routes::device::heartbeat))
        .route("/logs", post(crate::routes::device::report_log))
        .route("/bind", post(crate::routes::device::bind_device));

    let admin_routes = Router::new()
        .route("/provision", post(crate::routes::admin::provision_device));

    Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users))
        .route("/users/me/fcm-token", axum::routing::put(crate::routes::users::update_user_fcm_token))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .nest("/auth", auth_routes)
        .nest("/medicines", medicine_routes)
        .nest("/api/v1/devices", device_routes)
        .nest("/api/v1/admin", admin_routes)
        .with_state(state)
        .layer(cors)
}
