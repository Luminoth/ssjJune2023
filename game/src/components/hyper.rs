use std::sync::Arc;

use bevy::prelude::*;
use bevy_tokio_tasks::TaskContext;
use futures_lite::future;
use hyper::{Body, Request, Response};
use tokio::{sync::oneshot, task};

#[derive(/*Debug,*/ Component)]
pub struct StartHyperListener {
    pub port: u16,

    // TODO: can this be simplified here AND in the new() fn?
    #[allow(clippy::type_complexity)]
    pub request_handler: Arc<
        dyn Fn(
                u16,
                Request<Body>,
                TaskContext,
            ) -> future::Boxed<Result<Response<Body>, hyper::Error>>
            + Send
            + Sync,
    >,
}

impl StartHyperListener {
    pub fn new(
        port: u16,
        request_handler: impl Fn(
                u16,
                Request<Body>,
                TaskContext,
            ) -> future::Boxed<Result<Response<Body>, hyper::Error>>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self {
            port,
            request_handler: Arc::new(request_handler),
        }
    }
}

#[derive(Debug, Component)]
pub struct StopHyperListener(pub u16);

#[derive(Debug, Component)]
pub struct HyperTask(
    pub  (
        u16,
        Option<oneshot::Sender<bool>>,
        task::JoinHandle<Result<(), hyper::Error>>,
    ),
);
