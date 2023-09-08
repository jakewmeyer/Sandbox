use axum::{extract::State, http::HeaderMap, response::IntoResponse, routing::post, Router};
use reqwest::StatusCode;
use std::sync::Arc;
use stripe::Webhook;

use crate::error::Error;

use super::ApiContext;

pub fn routes() -> Router<Arc<ApiContext>> {
    Router::new().route("/v1/stripe/webhooks", post(stripe_webhook_handler))
}

// Handler for GET /v1/stripe/webhooks
async fn stripe_webhook_handler(
    State(ctx): State<Arc<ApiContext>>,
    headers: HeaderMap,
    body: String,
) -> Result<impl IntoResponse, Error> {
    let stripe_signature = headers
        .get("stripe-signature")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let stripe_webhook_secret = &ctx.config.stripe_webhook_secret;
    let event = Webhook::construct_event(&body, stripe_signature, &stripe_webhook_secret)?;
    let _event = Arc::new(event);
    Ok(StatusCode::OK)
}
