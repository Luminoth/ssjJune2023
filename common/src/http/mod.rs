use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticateRequest {
    pub oauth_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticateResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetUserResponse {
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct GetCharactersResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDuelRequest {
    pub character_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateDuelResponse {}
