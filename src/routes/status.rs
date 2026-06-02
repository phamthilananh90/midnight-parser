use axum::{routing::get, Router};
use serde::Serialize;

use crate::auth::ApiResponse;

pub fn routes() -> Router {
    Router::new().route("/health", get(health)).route("/ready", get(ready))
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
}

async fn health() -> ApiResponse<HealthResponse> {
    ApiResponse::ok(HealthResponse { status: "ok", version: env!("CARGO_PKG_VERSION") })
}

async fn ready() -> ApiResponse<HealthResponse> {
    ApiResponse::ok(HealthResponse { status: "ready", version: env!("CARGO_PKG_VERSION") })
}
