#![cfg(feature = "client")]

pub mod splash;

use bevy::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct Authorization {
    pub access_token: String,
    pub refresh_token: String,
}
