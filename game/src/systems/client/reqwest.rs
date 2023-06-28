use bevy::prelude::*;
use bevy_tokio_tasks::*;
use futures_lite::future;

use crate::components::client::reqwest::*;

pub fn start_http_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut ReqwestRequest), Added<ReqwestRequest>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter_mut() {
        let client = request.0 .0.clone();
        let request = request.0 .1.try_clone().unwrap();

        let task =
            runtime.spawn_background_task(|_ctx| async move { client.execute(request).await });

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
