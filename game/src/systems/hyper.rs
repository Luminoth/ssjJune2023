use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use futures_lite::future;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use tokio::sync::oneshot;

use crate::components::hyper::*;

async fn shutdown_signal(rx: oneshot::Receiver<bool>) {
    // TODO: error handling
    rx.await.unwrap();
}

pub fn start_http_listeners(
    mut commands: Commands,
    requests: Query<(Entity, &StartHyperListener), Added<StartHyperListener>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter() {
        let port = request.port;
        let request_handler = request.request_handler.clone();

        let (tx, rx) = oneshot::channel();

        let task = runtime.spawn_background_task(move |ctx| async move {
            let addr = ([127, 0, 0, 1], port).into();

            let service = make_service_fn(move |_| {
                let ctx = ctx.clone();
                let request_handler = request_handler.clone();
                async move {
                    Ok::<_, hyper::Error>(service_fn(move |req| {
                        let ctx = ctx.clone();
                        let request_handler = request_handler.clone();
                        (request_handler)(port, req, ctx)
                    }))
                }
            });

            let server = Server::bind(&addr).serve(service);
            let graceful = server.with_graceful_shutdown(shutdown_signal(rx));

            debug!("listening on http://{}", addr);

            graceful.await?;

            Ok(())
        });

        commands
            .entity(entity)
            .insert(HyperTask((port, Some(tx), task)))
            .remove::<StartHyperListener>();
    }
}

pub fn stop_http_listeners(
    mut commands: Commands,
    requests: Query<(Entity, &StopHyperListener), Added<StopHyperListener>>,
    mut tasks: Query<(Entity, &mut HyperTask)>,
) {
    for (entity, request) in requests.iter() {
        for (_, mut task) in tasks.iter_mut() {
            if task.0 .0 == request.0 {
                debug!("stopping listener on port {}", request.0);

                if let Some(tx) = task.0 .1.take() {
                    // TODO: error handling
                    tx.send(true).unwrap();
                }
            }
        }
        commands.entity(entity).despawn();
    }
}

pub fn poll_http_listeners(mut commands: Commands, mut tasks: Query<(Entity, &mut HyperTask)>) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(response) = future::block_on(future::poll_once(&mut task.0 .2)) {
            // TODO: error handling
            let response = response.unwrap();

            // TODO: error handling
            response.unwrap();

            debug!("hyper listener on port {} shut down", task.0 .0);

            commands.entity(entity).despawn();
        }
    }
}
