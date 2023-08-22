use bevy::prelude::*;

#[derive(Debug, Reflect, Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);
