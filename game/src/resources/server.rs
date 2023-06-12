#![cfg(feature = "server")]

use aws_config::SdkConfig;
use bevy::prelude::*;

use crate::cooldown::Throttle;

#[derive(Resource, Deref, DerefMut)]
pub struct AwsConfig(pub SdkConfig);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct AwsThrottle(pub Throttle);
