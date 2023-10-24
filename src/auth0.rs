use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, jwk::AlgorithmParameters, DecodingKey, Validation};
use jsonwebtoken::{
    jwk::{Jwk, JwkSet},
    TokenData,
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::error::Error;

/// An Auth0 client that can be used to decode JWT tokens
/// and call the Auth0 Management API.
#[derive(Debug, Clone)]
pub struct Client {
    domain: String,
    client_id: String,
    client_secret: String,
    jwk_cache: HashMap<String, Jwk>,
}

/// The claims in the JWT token
#[derive(Debug, Clone, Deserialize)]
pub struct AuthClaims {
    pub sub: String,
    pub permissions: Vec<String>,
}

impl Client {
    pub fn new(domain: String, client_id: String, client_secret: String) -> Self {
        Self {
            domain,
            client_id,
            client_secret,
            jwk_cache: HashMap::new(),
        }
    }

    /// Fetch and cache a JWK set from an Auth0 tenant
    pub async fn load_jwk(&mut self) -> Result<(), Error> {
        let url = format!("https://{}/.well-known/jwks.json", self.domain);
        let body = reqwest::get(url).await?.text().await?;
        let jwks: JwkSet = serde_json::from_str(&body)?;
        for jwk in jwks.keys {
            let kid = jwk
                .common
                .key_id
                .clone()
                .ok_or_else(|| anyhow!("No kid found"))?;
            self.jwk_cache.insert(kid, jwk.clone());
        }
        Ok(())
    }

    /// Validate and decode a JWT token using the cached JWK set
    pub fn decode_token(&self, kid: String, token: &str) -> Result<TokenData<AuthClaims>, Error> {
        if let Some(j) = &self.jwk_cache.get(&kid) {
            match j.algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .map_err(|_| Error::Auth0)?;
                    let validation = Validation::default();
                    let decoded_token = decode::<AuthClaims>(token, &decoding_key, &validation)
                        .map_err(|_| Error::Auth0)?;
                    Ok(decoded_token)
                }
                _ => Err(Error::Auth0),
            }
        } else {
            Err(Error::Auth0)
        }
    }

    /// Use the client credentials grant to get a management API access token
    async fn get_management_access_token(&self) -> Result<String, Error> {
        let url = format!("https://{}/oauth/token", self.domain);
        let audience = format!("https://{}/api/v2/", self.domain);
        let params = [
            ("client_id", self.client_id.as_str()),
            ("client_secret", self.client_secret.as_str()),
            ("audience", audience.as_str()),
            ("grant_type", "client_credentials"),
        ];
        let body = reqwest::Client::new()
            .post(url)
            .form(&params)
            .send()
            .await?
            .text()
            .await?;
        let token: serde_json::Value = serde_json::from_str(&body)?;
        let token = token
            .get("access_token")
            .ok_or_else(|| anyhow!("No access token found"))?
            .as_str()
            .ok_or_else(|| anyhow!("No access token found"))?;
        Ok(token.to_string())
    }
}
