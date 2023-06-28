use bevy::prelude::*;
use tokio::task;

#[derive(Debug, Component)]
pub struct ReqwestRequest(pub (reqwest::Client, reqwest::Request));

#[derive(Debug, Component)]
pub struct ReqwestTask(pub task::JoinHandle<Result<reqwest::Response, reqwest::Error>>);

#[derive(Debug, Component)]
pub struct ReqwestResult(pub Option<Result<reqwest::Response, reqwest::Error>>);
