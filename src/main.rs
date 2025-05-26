use axum::{
    Router, extract::Json, http::StatusCode, response::Json as ResponseJson, routing::{post, get},
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize)]
struct LogRequest {
    data: String,
}

#[derive(Serialize)]
struct LogResponse {
    status: String,
    message: String,
}

async fn post_log(
    Json(payload): Json<LogRequest>,
) -> Result<ResponseJson<LogResponse>, StatusCode> {
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let log_entry = format!("[{}] {}\n", timestamp, payload.data);

    tracing::info!("Received log entry: {}", payload.data);

    match write_to_log_file(&log_entry).await {
        Ok(_) => {
            tracing::info!("Logged: {}", payload.data);
            Ok(ResponseJson(LogResponse {
                status: "success".to_string(),
                message: "Log entry written successfully".to_string(),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to write to log file: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn write_to_log_file(log_entry: &str) -> std::io::Result<()> {
    let log_path = std::env::var("LOG_PATH").unwrap_or_else(|_| "server.log".to_string());
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;

    file.write_all(log_entry.as_bytes())?;
    file.flush()?;
    Ok(())
}

async fn health() -> &'static str {
    "OK"
}

#[tokio::main]
async fn main() {
    // Initialize detailed tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_level(true)
                .with_ansi(true),
        )
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("potmeme_worker=debug,potmeme_core=debug,info")),
        )
        .init();

    // Build our application with a route
    let app = Router::new()
        .route("/log", post(post_log))
        .route("/health", get(health))
        .layer(TraceLayer::new_for_http());

    // Run it with hyper on localhost:8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    tracing::info!("Starting server at http://0.0.0.0:8080");
    tracing::info!("Logs will be written to: server.log");

    axum::serve(listener, app).await.unwrap();
}
