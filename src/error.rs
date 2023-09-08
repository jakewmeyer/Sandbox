use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tracing::error;

/// Common Error type that allows us to return `Result` in handler functions.
///
/// User facing errors are defined with a corresponding status code and user
/// friendly message, while any one off `Anyhow` errors are automatically
/// considered to be 500, and the resulting error is only logged application
/// side for security purposes.
#[derive(Error, Debug)]
pub enum Error {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Not Found")]
    NotFound,

    #[error("Too Many Requests")]
    TooManyRequests,

    #[error("Auth0 Error")]
    Auth0,

    #[error("Stripe webhook error")]
    WebhookError(#[from] stripe::WebhookError),

    #[error("Stripe error")]
    StripeError(#[from] stripe::StripeError),

    #[error("An error occurred parsing JSON")]
    JsonRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error("An error occurred extracting the path")]
    PathRejection(#[from] axum::extract::rejection::PathRejection),

    #[error("An error occurred extracting the querystring")]
    QueryRejection(#[from] axum::extract::rejection::QueryRejection),

    #[error("A database error occurred")]
    DbErr(#[from] sea_orm::error::DbErr),

    #[error("An internal server error occurred")]
    Anyhow(#[from] anyhow::Error),

    #[error("A request error occurred")]
    RequestError(#[from] reqwest::Error),

    #[error("Serde error")]
    SerdeError(#[from] serde_json::Error),
}

/// Implement Axum's `IntoResponse` trait for our errors, so we can
/// return `Result` from handlers
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::JsonRejection(e) => e.into_response(),
            Self::PathRejection(e) => e.into_response(),
            Self::QueryRejection(e) => e.into_response(),
            Self::Unauthorized => StatusCode::UNAUTHORIZED.into_response(),
            Self::Forbidden => StatusCode::FORBIDDEN.into_response(),
            Self::NotFound => StatusCode::NOT_FOUND.into_response(),
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS.into_response(),
            Self::Auth0 => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
            Self::WebhookError(e) => {
                error!("Stripe webhook error: {:?}", e);
                StatusCode::BAD_REQUEST.into_response()
            }
            Self::StripeError(e) => {
                error!("Stripe error: {:?}", e);
                StatusCode::BAD_REQUEST.into_response()
            }
            Self::DbErr(e) => {
                error!("Database error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::Anyhow(ref e) => {
                error!("Anyhow error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::RequestError(ref e) => {
                error!("Request error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            Self::SerdeError(ref e) => {
                error!("Serde error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
