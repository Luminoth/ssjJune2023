use bevy::prelude::*;

#[derive(Event)]
pub struct RefreshAuthentication;

#[derive(Event)]
pub struct AuthenticationResult(pub bool);
