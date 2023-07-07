use bevy::prelude::*;
use tokio::task;

#[derive(Debug, Component)]
pub struct StartHyperListener(pub u16);

#[derive(Debug, Component)]
pub struct StopHyperListener(pub u16);

#[derive(Debug, Component)]
pub struct HyperTask(pub (u16, task::JoinHandle<Result<(), hyper::Error>>));

#[derive(Debug, Component)]
pub struct HyperRequest;
