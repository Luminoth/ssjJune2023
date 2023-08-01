use std::sync::Arc;

use bevy::prelude::*;
use bevy_tokio_tasks::TaskContext;
use futures_lite::future;
use tokio::task;

#[derive(/*Debug,*/ Component)]
pub struct ReqwestRequest {
    pub request: reqwest::Request,

    // TODO: can this be simplified here AND in the new() fn?
    #[allow(clippy::type_complexity)]
    pub request_handler: Arc<
        dyn Fn(Result<bytes::Bytes, reqwest::Error>, TaskContext) -> future::Boxed<()>
            + Send
            + Sync,
    >,
}

impl ReqwestRequest {
    pub fn new(
        request: reqwest::Request,
        request_handler: impl Fn(Result<bytes::Bytes, reqwest::Error>, TaskContext) -> future::Boxed<()>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self {
            request,
            request_handler: Arc::new(request_handler),
        }
    }
}

#[derive(Debug, Component)]
pub struct ReqwestTask(pub task::JoinHandle<()>);
