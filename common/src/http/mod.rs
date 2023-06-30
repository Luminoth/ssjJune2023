use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticateRequest {
    pub access_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticateResponse {
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct GetCharactersResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDuelRequest {
    pub character_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateDuelResponse {}
