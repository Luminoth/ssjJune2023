use bevy::prelude::*;

#[derive(Event)]
pub struct RefreshAuthentication;

#[derive(Event)]
pub struct AuthenticationSuccess;

#[derive(Event)]
pub struct AuthenticationFailure;
