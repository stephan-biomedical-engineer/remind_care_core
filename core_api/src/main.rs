use dotenvy::dotenv;
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;

use rust_raw_server::{app::{build_app, AppState}, config::Config, telemetry};

#[tokio::main]
async fn main() 
{
    dotenv().ok();

    let config = Config::from_env();

    telemetry::init(&config);

    info!("Iniciando o servidor (Ambiente: {})", config.app_env);
    info!("Conectando ao banco de dados...");

    let pool = PgPool::connect(&config.database_url)
        .await
        .expect("[INIT FATAL] Falha ao conectar ao PostgreSQL");

    info!("Rodando migrations do banco de dados...");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("[INIT FATAL] Falha ao executar migrations");

    let state = AppState 
    {
        pool,
        config, 
    };

    let app = build_app(state);

    let listener = TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("[INIT FATAL] Falha ao fazer o bind da porta 7878");

    info!("Servidor rodando em http://{}", listener.local_addr().unwrap());

    axum::serve
    (
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await
        .expect("[INIT FATAL] Servidor falhou");
}