#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

use anyhow::Result;
use dotenvy::dotenv;
use figment::providers::{Env, Serialized};
use figment::Figment;
use sandbox_api::api;
use sandbox_api::config::Config;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let config: Config = Figment::from(Serialized::defaults(Config::default()))
        .merge(Env::raw())
        .extract()?;

    api::serve(config).await?;

    Ok(())
}
