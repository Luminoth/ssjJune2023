use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Resource)]
pub enum AuthenticationState {
    // Oauth steps
    Unauthorized,
    WaitForAuthorization,

    // Game auth steps
    Unauthenticated,
    WaitForAuthentication,
    Authenticated,
    WaitForRefresh,
}

impl AuthenticationState {
    pub fn is_authenticated(&self) -> bool {
        match self {
            Self::Authenticated | Self::WaitForRefresh => true,
            _ => false,
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Resource)]
pub struct Authorization {
    // oauth access token
    #[serde(skip)]
    pub oauth_token: String,

    pub access_token: String,
    pub refresh_token: String,
}

pub type AuthorizationResource = Authorization; //Persistent<Authorization>;
