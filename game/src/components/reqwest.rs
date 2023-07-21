use bevy::prelude::*;
use tokio::task;

#[derive(Debug, Component)]
pub struct ReqwestRequest(pub reqwest::Request);

#[derive(Debug, Component)]
pub struct ReqwestTask(pub task::JoinHandle<Result<bytes::Bytes, reqwest::Error>>);

#[derive(Debug, Component)]
pub struct ReqwestResult(pub Option<Result<bytes::Bytes, reqwest::Error>>);
