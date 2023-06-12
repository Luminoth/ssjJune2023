#![cfg(feature = "client")]

use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SplashTimer(pub Timer);
