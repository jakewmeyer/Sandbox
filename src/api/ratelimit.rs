use std::time::Instant;

use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};

use crate::error::Error;

use super::ApiContext;

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

    fn take(&mut self, tokens: u8) -> bool {
        self.update();
        if self.available_tokens >= tokens {
            self.available_tokens -= tokens;
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
    State(ctx): State<ApiContext>,
    req: Request<B>,
    next: Next<B>,
) -> Result<impl IntoResponse, Error> {
    let ip = req
        .headers()
        .get("Fly-Client-IP")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("127.0.0.1");
    let rate_limit = &mut ctx.rate_limit.lock().await;
    let bucket = rate_limit.entry(ip.to_string()).or_insert_with(|| {
        TokenBucket::new(
            ctx.config.rate_limit_capacity,
            ctx.config.rate_limit_fill_rate,
        )
    });
    if bucket.take(1) {
        Ok(next.run(req).await)
    } else {
        Err(Error::TooManyRequests)
    }
}
