use bevy::prelude::*;
use bevy_tokio_tasks::TaskContext;
use futures_lite::future;
use tokio::task;

pub type AsyncResponseHandler = std::sync::Arc<
    dyn Fn(Result<bytes::Bytes, reqwest::Error>, TaskContext) -> future::Boxed<()> + Send + Sync,
>;

#[derive(/*Debug,*/ Component)]
pub struct ReqwestRequest(pub (reqwest::Request, AsyncResponseHandler));

#[derive(Debug, Component)]
pub struct ReqwestTask(pub task::JoinHandle<()>);
