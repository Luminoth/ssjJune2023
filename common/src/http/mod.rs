use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticateRequest {
    pub access_token: String,
}

#[derive(Debug, Serialize)]
pub struct GetCharactersResponse {}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDuelRequest {
    pub user_id: Uuid,
    pub character_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateDuelResponse {}
