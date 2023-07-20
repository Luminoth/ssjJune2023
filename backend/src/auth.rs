use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use common::auth::*;

fn generate_token(claims: &Claims, secret: impl AsRef<[u8]>) -> anyhow::Result<String> {
    let header = Header::default();
    Ok(encode(
        &header,
        claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?)
}

pub fn generate_tokens_for_user(
    subject: impl Into<String>,
    secret: impl AsRef<[u8]>,
) -> anyhow::Result<(String, String)> {
    let subject = subject.into();

    let access_claims = Claims::new(
        subject.clone(),
        "ssjJune2023-issuer",
        "ssjJune2023-user",
        ACCESS_TOKEN_TTL,
    );

    let refresh_claims = Claims::new(
        subject,
        "ssjJune2023-issuer",
        "ssjJune2023-user-refresh",
        REFRESH_TOKEN_TTL,
    );

    Ok((
        generate_token(&access_claims, &secret)?,
        generate_token(&refresh_claims, secret)?,
    ))
}

pub fn validate_user_access_token(
    token: impl AsRef<str>,
    secret: impl AsRef<[u8]>,
) -> anyhow::Result<String> {
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

pub fn validate_user_refresh_token(
    token: impl AsRef<str>,
    secret: impl AsRef<[u8]>,
) -> anyhow::Result<String> {
    let mut validation = Validation::default();
    validation.set_issuer(&["ssjJune2023-issuer"]);
    validation.set_audience(&["ssjJune2023-user-refresh"]);

    let token = decode::<Claims>(
        token.as_ref(),
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    Ok(token.claims.sub)
}
