use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "liyo-server"
    }))
}

pub fn router() -> Router {
    Router::new().route("/health", get(health))
}
