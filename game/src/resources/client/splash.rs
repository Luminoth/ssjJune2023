use bevy::prelude::*;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);
