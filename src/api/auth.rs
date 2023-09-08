use std::sync::Arc;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts, HeaderValue},
};
use jsonwebtoken::decode_header;

use crate::{entity::users, error::Error};

use super::{
    users::{create_user, get_user_by_provider_id, CreateUser},
    ApiContext,
};

#[derive(Debug)]
pub struct AuthUser {
    pub user: users::Model,
    pub user_id: String,
    pub permissions: Vec<String>,
}

impl AuthUser {
    async fn from_authorization(
        ctx: &Arc<ApiContext>,
        auth_header: &HeaderValue,
    ) -> Result<Self, Error> {
        let token = auth_header
            .to_str()
            .map_err(|_| Error::Unauthorized)?
            .strip_prefix("Bearer ")
            .ok_or(Error::Unauthorized)?;
        let header = decode_header(token).map_err(|_| Error::Unauthorized)?;
        let kid = match header.kid {
            Some(k) => k,
            None => return Err(Error::Unauthorized),
        };
        let decoded_token = ctx.auth0_client.decode_token(kid, token)?;
        let user_id = decoded_token.claims.sub;
        let user = get_user_by_provider_id(ctx, &user_id).await;
        let user = match user {
            Ok(Some(user)) => user,
            Ok(None) => create_user(
                ctx,
                CreateUser {
                    provider_id: user_id.clone(),
                },
            )
            .await
            .map_err(|_| Error::Unauthorized)?,
            Err(_) => return Err(Error::Unauthorized),
        };
        let permissions = decoded_token.claims.permissions;
        Ok(Self {
            user,
            user_id,
            permissions,
        })
    }
    pub fn has_permission(&self, permission: &str) -> Result<bool, Error> {
        match self
            .permissions
            .iter()
            .find(|p| p == &&permission.to_string())
        {
            Some(_) => Ok(true),
            None => Err(Error::Forbidden),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    Arc<ApiContext>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Error;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ctx = Arc::from_ref(state);
        if let Some(auth_header) = parts.headers.get(header::AUTHORIZATION) {
            Ok(Self::from_authorization(&ctx, auth_header).await?)
        } else {
            Err(Error::Unauthorized)
        }
    }
}
