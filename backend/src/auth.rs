use chrono::prelude::*;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    iss: String,
    aud: String,
}

impl Claims {
    pub fn new(
        subject: impl Into<String>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
        ttl: i64,
    ) -> Self {
        Self {
            sub: subject.into(),
            exp: (Utc::now().timestamp() + ttl) as usize,
            iss: issuer.into(),
            aud: audience.into(),
        }
    }
}

fn generate_token(claims: &Claims, secret: impl AsRef<[u8]>) -> anyhow::Result<String> {
    let header = Header::default();
    Ok(encode(
        &header,
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?)
}

pub fn generate_token_for_user(
    subject: impl Into<String>,
    secret: impl AsRef<[u8]>,
) -> anyhow::Result<String> {
    let claims = Claims::new(
        subject.into(),
        "ssjJune2023-issuer",
        "ssjJune2023-user",
        86400,
    );
    generate_token(&claims, secret)
}

#[allow(dead_code)]
pub fn validate_token(token: impl AsRef<str>, secret: impl AsRef<[u8]>) -> anyhow::Result<String> {
    let mut validation = Validation::default();
    validation.set_issuer(&["ssjJune2023-issuer"]);
    validation.set_audience(&["ssjJune2023-user"]);

    let token = decode::<Claims>(
        token.as_ref(),
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    Ok(token.claims.sub)
}
