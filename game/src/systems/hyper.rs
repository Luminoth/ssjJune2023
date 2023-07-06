use bevy::prelude::*;
use bevy_tokio_tasks::*;
use futures_lite::future;
use hyper::{
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};

use crate::components::hyper::*;

// TODO: how do we *stop* listeners?

async fn http_request_handler(
    req: Request<Body>,
    mut ctx: TaskContext,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            info!("got POST to '/'");

            ctx.run_on_main_thread(|_ctx| {
                info!("hello main thread!");
            })
            .await;

            Ok(Response::default())
        }
        _ => {
            info!("http listener returning not found");

            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub fn start_http_listener(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut HyperListen), Added<HyperListen>>,
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

            info!("listening on http://{}", addr);

            server.await?;

            Ok(())
        });

        commands
            .entity(entity)
            .insert(HyperTask(task))
            .remove::<HyperListen>();
    }
}

pub fn poll_http_listeners(mut commands: Commands, mut requests: Query<(Entity, &mut HyperTask)>) {
    for (entity, mut task) in requests.iter_mut() {
        if let Some(response) = future::block_on(future::poll_once(&mut task.0)) {
            // TODO: error handling
            let _response = response.unwrap();

            commands.entity(entity).remove::<HyperTask>();
        }
    }
}
