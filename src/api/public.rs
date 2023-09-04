use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

use super::ApiContext;

pub fn routes() -> Router<ApiContext> {
    Router::new().route("/health", get(healthcheck))
}

// Handler for GET /health
pub async fn healthcheck() -> impl IntoResponse {
    StatusCode::OK
}
