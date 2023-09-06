//! http contains a serve function that constructs a new
//! Axum app from a Config and attempts to serve it

use crate::auth0::Client;
use crate::config::Config;
use ::stripe::Client as StripeClient;
use ::stripe::RequestStrategy::ExponentialBackoff;
use anyhow::Result;
use axum::{middleware, Router};
use sea_orm::{Database, DatabaseConnection};
use tower_http::timeout::TimeoutLayer;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::sync::Mutex;
use tower_default_headers::DefaultHeadersLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;
use tracing::info;

use self::ratelimit::TokenBucket;

mod accounts;
mod auth;
mod pagination;
mod public;
mod ratelimit;
mod stripe;
mod users;

#[derive(Clone)]
pub struct ApiContext {
    db: DatabaseConnection,
    config: Config,
    rate_limit: Arc<Mutex<HashMap<String, TokenBucket>>>,
    stripe_client: StripeClient,
    auth0_client: Client,
}

/// Creates a signal handler for graceful shutdown.
async fn shutdown_signal() {
    // Handle SIGINT
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // Handle SIGTERM
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    // Any other graceful shutdow logic goes here
    info!("Signal received, starting graceful shutdown...");
}

/// Create and serve an Axum server with pre-registered routes
/// and middleware
pub async fn serve(config: Config) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;

    let db = Database::connect(&config.database_url).await?;

    let stripe_client =
        StripeClient::new(&config.stripe_secret_key).with_strategy(ExponentialBackoff(5));

    let mut auth0_client = Client::new(
        config.auth0_domain.clone(),
        config.auth0_client_id.clone(),
        config.auth0_client_secret.clone(),
    );
    auth0_client.load_jwk().await?;

    let state = ApiContext {
        config: config.clone(),
        db,
        rate_limit: Arc::new(Mutex::new(HashMap::new())),
        stripe_client,
        auth0_client,
    };

    let app = Router::new()
        .merge(public::routes())
        .merge(accounts::routes())
        .merge(users::routes())
        .merge(stripe::routes())
        .layer(TimeoutLayer::new(Duration::from_secs(config.request_timeout)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            ratelimit::limiter,
        ))
        .layer(CorsLayer::new())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(DefaultHeadersLayer::new(owasp_headers::headers()))
        .with_state(state);

    info!("Listening on {}", addr);
    axum::Server::try_bind(&addr)?
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
