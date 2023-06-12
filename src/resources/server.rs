#![cfg(feature = "server")]

use aws_config::SdkConfig;
use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct AwsConfig(pub SdkConfig);
