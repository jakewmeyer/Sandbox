//! Application config parameters
//!
//! Config args can be passed via environment variables.
//! Dotenv support is included in main.

use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DurationSeconds;

impl Default for Config {
    fn default() -> Self {
        Config {
            host: String::from("0.0.0.0"),
            port: String::from("5678"),
            request_timeout: Duration::from_secs(30),
            database_url: String::new(),
            database_timeout: Duration::from_secs(60),
            stripe_secret_key: String::new(),
            stripe_webhook_secret: String::new(),
            auth0_domain: String::new(),
            auth0_client_id: String::new(),
            auth0_client_secret: String::new(),
            rate_limit_capacity: 100,
            rate_limit_fill_rate: 1,
        }
    }
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// The host the server will bind to, any valid
    /// IpAddr will suffice
    pub host: String,

    // The port the server will bind to
    pub port: String,

    // Request timeout duration
    #[serde_as(as = "DurationSeconds<u64>")]
    pub request_timeout: Duration,

    // Database connection string
    pub database_url: String,

    // Database connection timeout
    #[serde_as(as = "DurationSeconds<u64>")]
    pub database_timeout: Duration,

    // Stripe secret key
    pub stripe_secret_key: String,

    // Stripe webhook secret
    pub stripe_webhook_secret: String,

    // Auth0 domain
    pub auth0_domain: String,

    // Auth0 client id
    pub auth0_client_id: String,

    // Auth0 client secret
    pub auth0_client_secret: String,

    // Rate limit bucket capacity
    pub rate_limit_capacity: u8,

    // Rate limit bucket fill rate per second
    pub rate_limit_fill_rate: u8,
}
