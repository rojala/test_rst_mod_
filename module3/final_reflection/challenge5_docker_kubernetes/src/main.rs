use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    service: String,
    version: String,
    uptime_secs: u64,
}

#[derive(Serialize, Deserialize)]
struct Message {
    id: u64,
    text: String,
}

static START_TIME: OnceLock<u64> = OnceLock::new();
static MESSAGE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize start time
    let _ = START_TIME.get_or_init(|| {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    });

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/messages", post(create_message))
        .route("/api/messages/:id", get(get_message))
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind port 3000");

    println!("Server listening on 0.0.0.0:3000");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn root() -> &'static str {
    "Rust Web Service - Endpoints: /health, POST /api/messages, GET /api/messages/:id"
}

async fn health_check() -> impl IntoResponse {
    let start_time = *START_TIME.get_or_init(|| 0);
    let uptime = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - start_time;

    let response = HealthResponse {
        status: "ok".to_string(),
        service: "rust-web-service".to_string(),
        version: "0.1.0".to_string(),
        uptime_secs: uptime,
    };

    (StatusCode::OK, Json(response))
}

async fn create_message(Json(payload): Json<Message>) -> impl IntoResponse {
    let id = MESSAGE_COUNTER.fetch_add(1, Ordering::SeqCst);
    let message = Message {
        id,
        text: payload.text,
    };

    tracing::info!("Created message: {:?}", message);
    (StatusCode::CREATED, Json(message))
}

async fn get_message(Path(id): Path<u64>) -> impl IntoResponse {
    let message = Message {
        id,
        text: format!("Message {}", id),
    };

    tracing::info!("Retrieved message: {:?}", message);
    (StatusCode::OK, Json(message))
}
