#![cfg(feature = "server")]

use aws_config::SdkConfig;
use bevy::prelude::*;
use bevy::tasks::*;

#[derive(Component)]
pub struct OnInitServer;

#[derive(Component)]
pub struct AwsConfigTask(pub Task<SdkConfig>);
