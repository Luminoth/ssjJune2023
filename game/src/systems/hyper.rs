use bevy::prelude::*;
use bevy_tokio_tasks::*;
use futures_lite::future;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};

use crate::components::hyper::*;

async fn shutdown_signal() {
    // TODO: replace this with a channel
    tokio::time::sleep(tokio::time::Duration::from_millis(68719476734)).await;
}

async fn http_request_handler(
    req: Request<Body>,
    mut ctx: TaskContext,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            // TODO: no token to be found here, need to serve up a page
            // that gets the token and then feeds it back in on another route

            /*
            var queryString = window.location.hash.slice(1);
            var params = new URLSearchParams(queryString);
            var accessToken = params.get("access_token");
            */

            info!("got GET to '/': {:?}", req.uri());

            ctx.run_on_main_thread(|_ctx| {
                info!("GET on main thread!");
            })
            .await;

            Ok(Response::default())
        }
        _ => {
            info!("http listener returning not found: {:?}", req);

            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub fn start_http_listeners(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut StartHyperListener), Added<StartHyperListener>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter_mut() {
        let port = request.0;

        let task = runtime.spawn_background_task(move |ctx| async move {
            let addr = ([127, 0, 0, 1], port).into();

            let service = make_service_fn(move |_| {
                let ctx = ctx.clone();
                async move {
                    Ok::<_, hyper::Error>(service_fn(move |req| {
                        let ctx = ctx.clone();
                        http_request_handler(req, ctx)
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
