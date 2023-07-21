use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use futures_lite::future;

use crate::components::reqwest::*;
use crate::resources::reqwest::*;

// TODO: instead of using error_for_status,
// we probably want to pass the status back to the handler

// TODO: do we need a way to cancel requests?

pub fn start_http_requests(
    mut commands: Commands,
    client: Res<ReqwestClient>,
    requests: Query<(Entity, &ReqwestRequest), Added<ReqwestRequest>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter() {
        let client = client.clone();
        let request = request.0.try_clone().unwrap();

        let task = runtime.spawn_background_task(|_ctx| async move {
            client
                .execute(request)
                .await?
                .error_for_status()?
                .bytes()
                .await
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
            let response = response.unwrap();

            commands
                .entity(entity)
                .insert(ReqwestResult(Some(response)))
                .remove::<ReqwestTask>();
        }
    }
}
