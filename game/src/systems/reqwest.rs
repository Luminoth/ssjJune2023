use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use futures_lite::future;

use crate::components::reqwest::*;
use crate::resources::reqwest::*;

// TODO: do we need a way to cancel requests?

pub fn start_http_requests(
    mut commands: Commands,
    client: Res<ReqwestClient>,
    requests: Query<(Entity, &ReqwestRequest), Added<ReqwestRequest>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter() {
        let client = client.clone();
        let reqwest_request = request.request.try_clone().unwrap();
        let response_handler = request.request_handler.clone();

        let task = runtime.spawn_background_task(|ctx| async move {
            match client.execute(reqwest_request).await {
                Ok(response) => {
                    // TODO: instead of using error_for_status,
                    // we probably want to pass the status back to the handler?
                    match response.error_for_status() {
                        Ok(response) => {
                            (response_handler)(response.bytes().await, ctx).await;
                        }
                        Err(e) => {
                            (response_handler)(Err(e), ctx).await;
                        }
                    }
                }
                Err(e) => {
                    (response_handler)(Err(e), ctx).await;
                }
            }
        });

        commands
            .entity(entity)
            .insert(ReqwestTask(task))
            .remove::<ReqwestRequest>();
    }
}

pub fn poll_http_requests(mut commands: Commands, mut requests: Query<(Entity, &mut ReqwestTask)>) {
    for (entity, mut task) in requests.iter_mut() {
        if let Some(response) = future::block_on(future::poll_once(&mut task.0)) {
            // TODO: error handling
            response.unwrap();

            commands.entity(entity).despawn();
        }
    }
}
