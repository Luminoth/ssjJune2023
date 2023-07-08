use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use futures_lite::future;
use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};

use crate::components::hyper::*;

async fn shutdown_signal() {
    // TODO: replace this with a channel
    tokio::time::sleep(tokio::time::Duration::from_millis(68719476734)).await;
}

pub fn start_http_listeners(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut StartHyperListener), Added<StartHyperListener>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter_mut() {
        let port = request.0 .0;
        let request_handler = request.0 .1.clone();

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
            let graceful = server.with_graceful_shutdown(shutdown_signal());

            info!("listening on http://{}", addr);

            graceful.await?;

            Ok(())
        });

        commands
            .entity(entity)
            .insert(HyperTask((port, task)))
            .remove::<StartHyperListener>();
    }
}

pub fn stop_http_listeners(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut StopHyperListener), Added<StopHyperListener>>,
    mut tasks: Query<(Entity, &mut HyperTask)>,
    //runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter_mut() {
        for (_, task) in tasks.iter_mut() {
            if task.0 .0 == request.0 {
                info!("stopping listener on port {}", request.0);

                // TODO: signal the channel to shutdown the listener
            }
        }
        commands.entity(entity).despawn();
    }
}

pub fn poll_http_listeners(mut commands: Commands, mut tasks: Query<(Entity, &mut HyperTask)>) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(response) = future::block_on(future::poll_once(&mut task.0 .1)) {
            // TODO: error handling
            let _response = response.unwrap();

            info!("hyper listener on port {} shut down", task.0 .0);

            commands.entity(entity).remove::<HyperTask>();
        }
    }
}
