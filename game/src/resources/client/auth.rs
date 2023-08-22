#![allow(dead_code)]

use bevy::prelude::*;
use bevy_persistent::prelude::*;
use chrono::prelude::*;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

use common::auth::*;

#[derive(Debug, Default, Reflect, Resource)]
pub enum AuthenticationState {
    // Oauth steps
    #[default]
    Unauthorized,
    WaitForAuthorization,

    // Game auth steps
    Unauthenticated,
    WaitForAuthentication,
    Authenticated,
    WaitForRefresh,
}

impl AuthenticationState {
    pub fn is_authorized(&self) -> bool {
        matches!(
            self,
            Self::Unauthenticated
                | Self::WaitForAuthentication
                | Self::Authenticated
                | Self::WaitForRefresh
        )
    }

    pub fn is_authenticated(&self) -> bool {
        matches!(self, Self::Authenticated | Self::WaitForRefresh)
    }
}

#[derive(Debug, Default, Reflect, Resource)]
pub struct AuthenticationError(pub Option<String>);

#[derive(Debug, Default, Deserialize, Serialize, Reflect, Resource)]
pub struct Authorization {
    #[serde(skip)]
    oauth_token: String,

    access_token: String,
    refresh_token: String,

    // TODO: if serde ever has a "finalize" step
    // we can just calculate this on deserialize
    // and avoid the need to have a jank cell setup
    #[serde(skip)]
    #[reflect(ignore)]
    token_expiry: RwLock<(u64, u64)>,
}

impl Authorization {
    pub fn has_oauth(&self) -> bool {
        !self.oauth_token.trim().is_empty()
    }

    pub fn get_oauth_token(&self) -> &String {
        &self.oauth_token
    }

    pub fn set_oauth_token(&mut self, token: impl Into<String>) {
        self.oauth_token = token.into()
    }

    pub fn clear_oauth_token(&mut self) {
        self.oauth_token.clear()
    }

    fn update_token_expiry(&self) {
        let mut expiry = self.token_expiry.write();
        expiry.0 = get_token_expiry(&self.access_token).unwrap_or_default();
        expiry.1 = get_token_expiry(&self.refresh_token).unwrap_or_default();
    }

    pub fn is_access_token_expired(&self) -> bool {
        let expiry = self.token_expiry.read();
        if expiry.0 == 0 {
            drop(expiry);
            self.update_token_expiry();

            return self.token_expiry.read().0 < Utc::now().timestamp() as u64;
        }
        expiry.0 < Utc::now().timestamp() as u64
    }

    pub fn should_refresh_access_token(&self) -> bool {
        let expiry = self.token_expiry.read();
        if expiry.0 == 0 {
            drop(expiry);
            self.update_token_expiry();

            return self.token_expiry.read().0
                <= (Utc::now().timestamp() + (ACCESS_TOKEN_TTL as f32 * 0.1) as i64) as u64;
        }
        expiry.0 <= (Utc::now().timestamp() + (ACCESS_TOKEN_TTL as f32 * 0.1) as i64) as u64
    }

    pub fn get_access_token(&self) -> &String {
        &self.access_token
    }

    pub fn is_refresh_token_expired(&self) -> bool {
        let expiry = self.token_expiry.read();
        if expiry.0 == 0 {
            drop(expiry);
            self.update_token_expiry();

            return self.token_expiry.read().1 < Utc::now().timestamp() as u64;
        }
        expiry.1 <= Utc::now().timestamp() as u64
    }

    pub fn get_refresh_token(&self) -> &String {
        &self.refresh_token
    }

    pub fn set_auth_tokens(
        &mut self,
        access_token: impl Into<String>,
        refresh_token: impl Into<String>,
    ) {
        self.access_token = access_token.into();
        self.refresh_token = refresh_token.into();

        self.update_token_expiry();
    }
}

pub type AuthorizationResource = Persistent<Authorization>;
