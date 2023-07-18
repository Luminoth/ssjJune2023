use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use futures_lite::future;

use crate::components::server::aws::*;

pub fn start_aws_requests<R>(
    mut commands: Commands,
    requests: Query<(Entity, &R), Added<R>>,
    runtime: Res<TokioTasksRuntime>,
) where
    R: AwsTaskRequest,
{
    for (entity, request) in requests.iter() {
        let request = request.clone();
        let task = runtime.spawn_background_task(|_ctx| async move { request.run().await });

        commands
            .entity(entity)
            .insert(AwsTask::<<R as AwsTaskRequest>::Output>::new(task))
            .remove::<R>();
    }
}

pub fn poll_aws_tasks<R>(mut commands: Commands, mut requests: Query<(Entity, &mut AwsTask<R>)>)
where
    R: Send + Sync + 'static,
{
    for (entity, mut task) in requests.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.get_handle_mut())) {
            // TODO: error handling
            let result = result.unwrap();

            commands
                .entity(entity)
                .insert(AwsTaskResult::new(Some(result)))
                .remove::<AwsTask<R>>();
        }
    }
}
