use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
    time::Instant,
};

use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};

use crate::error::Error;

use super::ApiContext;

const TAKE_RATE: u8 = 1;
const IP_HEADER: &str = "X-Real-IP";
const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Debug, Clone, Copy)]
pub struct TokenBucket {
    capacity: u8,
    available_tokens: u8,
    last_update: Instant,
    fill_rate: u8,
}

impl TokenBucket {
    fn new(capacity: u8, fill_rate: u8) -> Self {
        Self {
            capacity,
            available_tokens: capacity,
            last_update: Instant::now(),
            fill_rate,
        }
    }

    fn take(&mut self) -> bool {
        self.update();
        if self.available_tokens >= TAKE_RATE {
            self.available_tokens -= TAKE_RATE;
            true
        } else {
            false
        }
    }

    fn update(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update).as_secs();
        let tokens_to_add = (elapsed as u8) * self.fill_rate;
        // Check if we have at least one token to add
        // to prevent fractional token tracking
        if tokens_to_add >= 1 {
            self.available_tokens = (self.available_tokens + tokens_to_add).min(self.capacity);
            self.last_update = now;
        }
    }
}

pub async fn limiter<B>(
    State(ctx): State<Arc<ApiContext>>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, Error> {
    let ip = req
        .headers()
        .get(IP_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_IP);
    let mut bucket = ctx.rate_limit.entry(ip).or_insert_with(|| {
        TokenBucket::new(
            ctx.config.rate_limit_capacity,
            ctx.config.rate_limit_fill_rate,
        )
    });
    if bucket.take() {
        Ok(next.run(req).await)
    } else {
        Err(Error::TooManyRequests)
    }
}
