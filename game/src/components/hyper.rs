use bevy::prelude::*;
use tokio::task;

#[derive(Debug, Component)]
pub struct HyperListen(pub u16);

#[derive(Debug, Component)]
pub struct HyperTask(pub task::JoinHandle<Result<(), hyper::Error>>);

#[derive(Debug, Component)]
pub struct HyperRequest;
