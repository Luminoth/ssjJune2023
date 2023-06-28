use bevy::prelude::*;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct AuthenticationToken(pub String);
