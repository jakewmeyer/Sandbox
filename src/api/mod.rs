//! http contains a serve function that constructs a new
//! Axum app from a Config and attempts to serve it

use crate::auth0::Client;
use crate::config::Config;
use crate::token_bucket::TokenBucket;
use ::stripe::Client as StripeClient;
use ::stripe::RequestStrategy::ExponentialBackoff;
use anyhow::Result;
use axum::{middleware, Router};
use dashmap::DashMap;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use tokio::{select, signal};
use tower_default_headers::DefaultHeadersLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

mod accounts;
mod auth;
mod pagination;
mod public;
mod ratelimit;
mod stripe;
mod users;

pub struct ApiContext {
    db: DatabaseConnection,
    config: Config,
    rate_limit: DashMap<IpAddr, TokenBucket>,
    stripe_client: StripeClient,
    auth0_client: Client,
}

/// Creates a signal handler for graceful shutdown.
async fn shutdown_signal(ctx: Arc<ApiContext>) {
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

    select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    // Any other graceful shutdow logic goes here
    info!("Signal received, starting graceful shutdown...");
    ctx.db.clone().close().await.unwrap_or_else(|e| {
        error!("Failed to close database connection: {}", e);
    });
    info!("Graceful shutdown complete");
}

/// Create and serve an Axum server with pre-registered routes
/// and middleware
pub async fn serve(config: Config) -> Result<()> {
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;

    let mut opts = ConnectOptions::new(&config.database_url);
    opts.connect_timeout(config.database_timeout)
        .sqlx_logging(false);

    let db = Database::connect(opts).await?;

    let stripe_client =
        StripeClient::new(&config.stripe_secret_key).with_strategy(ExponentialBackoff(5));

    let mut auth0_client = Client::new(
        config.auth0_domain.clone(),
        config.auth0_client_id.clone(),
        config.auth0_client_secret.clone(),
    );
    auth0_client.load_jwk().await?;

    let state = Arc::new(ApiContext {
        config: config.clone(),
        db,
        rate_limit: DashMap::new(),
        stripe_client,
        auth0_client,
    });

    let app = Router::new()
        .merge(public::routes())
        .merge(accounts::routes())
        .merge(users::routes())
        .merge(stripe::routes())
        .layer(TimeoutLayer::new(config.request_timeout))
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
        .with_state(state.clone());

    info!("Listening on {}", addr);
    axum::Server::try_bind(&addr)?
        .http1_header_read_timeout(config.request_timeout)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(state.clone()))
        .await?;
    Ok(())
}
