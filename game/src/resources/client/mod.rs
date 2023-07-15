#![cfg(feature = "client")]

pub mod splash;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize, Resource)]
pub struct Authorization {
    // oauth access token
    #[serde(skip)]
    pub access_token: String,

    pub auth_token: String,
    pub refresh_token: String,
}
