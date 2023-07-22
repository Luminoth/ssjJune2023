use bevy::prelude::*;
use bevy_tokio_tasks::TaskContext;
use futures_lite::future;
use hyper::{Body, Request, Response};
use tokio::{sync::oneshot, task};

pub type AsyncRequestHandler = std::sync::Arc<
    dyn Fn(u16, Request<Body>, TaskContext) -> future::Boxed<Result<Response<Body>, hyper::Error>>
        + Send
        + Sync,
>;

#[derive(/*Debug,*/ Component)]
pub struct StartHyperListener(pub (u16, AsyncRequestHandler));

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
