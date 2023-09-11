use std::collections::HashMap;

use axum::{
    async_trait,
    extract::{FromRequestParts, Query},
    http::request::Parts,
    RequestPartsExt,
};
use uuid::Uuid;

use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Pagination {
    pub limit: u64,
    pub after: Uuid,
}

#[async_trait]
impl<S> FromRequestParts<S> for Pagination
where
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Query(params) = parts.extract::<Query<HashMap<String, String>>>().await?;
        let limit = params
            .get("limit")
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(10)
            .min(100);
        let after = params
            .get("after")
            .and_then(|s| s.parse::<Uuid>().ok())
            .unwrap_or(Uuid::nil());
        Ok(Pagination { limit, after })
    }
}
