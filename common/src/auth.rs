use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

pub const ACCESS_TOKEN_TTL: u64 = 3600;
pub const REFRESH_TOKEN_TTL: u64 = 604800;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
    pub iss: String,
    pub aud: String,
}

impl Claims {
    pub fn new(
        subject: impl Into<String>,
        issuer: impl Into<String>,
        audience: impl Into<String>,
        ttl: u64,
    ) -> Self {
        Self {
            sub: subject.into(),
            exp: jsonwebtoken::get_current_timestamp() + ttl,
            iss: issuer.into(),
            aud: audience.into(),
        }
    }
}

pub fn get_token_expiry(token: impl AsRef<str>) -> anyhow::Result<u64> {
    let mut validation = Validation::default();
    validation.insecure_disable_signature_validation();

    let token = decode::<Claims>(token.as_ref(), &DecodingKey::from_secret(&[]), &validation)?;
    Ok(token.claims.exp)
}
