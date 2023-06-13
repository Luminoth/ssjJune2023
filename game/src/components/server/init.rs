use aws_config::SdkConfig;
use bevy::prelude::*;
use tokio::task;

#[derive(Debug, Component)]
pub struct OnInitServer;

#[derive(Debug, Component)]
pub struct AwsConfigTask(pub task::JoinHandle<SdkConfig>);
