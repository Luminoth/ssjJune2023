use aws_config::SdkConfig;
use bevy::prelude::*;
use tokio::task;

#[derive(Component)]
pub struct OnInitServer;

#[derive(Component)]
pub struct AwsConfigTask(pub task::JoinHandle<SdkConfig>);
