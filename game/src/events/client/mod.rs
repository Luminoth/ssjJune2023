#![cfg(feature = "client")]

pub mod auth;

use bevy::prelude::*;

#[derive(Event)]
pub struct UserUpdated;
