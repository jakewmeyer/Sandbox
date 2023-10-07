use std::{
    net::{IpAddr, Ipv4Addr},
    sync::Arc,
};

use axum::{extract::State, http::Request, middleware::Next, response::IntoResponse};

use crate::{error::Error, token_bucket::TokenBucket};

use super::ApiContext;

const IP_HEADER: &str = "X-Real-IP";
const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

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
            ctx.config.rate_limit_take_rate,
        )
    });
    if bucket.take() {
        Ok(next.run(req).await)
    } else {
        Err(Error::TooManyRequests)
    }
}
