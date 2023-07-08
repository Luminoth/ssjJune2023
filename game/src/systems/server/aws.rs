use bevy::prelude::*;
use bevy_tokio_tasks::{TaskContext, TokioTasksRuntime};
use futures_lite::future;

use crate::components::server::aws::*;

pub fn start_load_aws_config_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut LoadAwsConfigRequest), Added<LoadAwsConfigRequest>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, _) in requests.iter_mut() {
        info!("loading AWS config...");

        let task =
            runtime.spawn_background_task(|_ctx| async move { aws_config::load_from_env().await });

        commands
            .entity(entity)
            .insert(LoadAwsConfigTask(task))
            .remove::<LoadAwsConfigRequest>();
    }
}

pub fn poll_load_aws_config_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut LoadAwsConfigTask)>,
) {
    for (entity, mut task) in requests.iter_mut() {
        if let Some(config) = future::block_on(future::poll_once(&mut task.0)) {
            // TODO: error handling
            let config = config.unwrap();

            debug!("loaded AWS config: {:?}", config);

            commands
                .entity(entity)
                .insert(LoadAwsConfigResult(Some(config)))
                .remove::<LoadAwsConfigTask>();
        }
    }
}

pub fn start_sqs_get_queue_url_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut SQSGetQueueUrlRequest), Added<SQSGetQueueUrlRequest>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter_mut() {
        let client = request.0 .0.clone();
        let queue_name = request.0 .1.clone();

        info!("getting SQS queue {} URL...", queue_name);

        let task = runtime.spawn_background_task(|_ctx| async move {
            client.get_queue_url().queue_name(queue_name).send().await
        });

        commands
            .entity(entity)
            .insert(SQSGetQueueUrlTask(task))
            .remove::<SQSGetQueueUrlRequest>();
    }
}

pub fn poll_sqs_get_url_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut SQSGetQueueUrlTask)>,
) {
    for (entity, mut task) in requests.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            // TODO: error handling
            let result = result.unwrap();

            commands
                .entity(entity)
                .insert(SQSGetQueueUrlResult(Some(result)))
                .remove::<SQSGetQueueUrlTask>();
        }
    }
}

pub fn start_sqs_receive_message_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut SQSReceiveMessageRequest), Added<SQSReceiveMessageRequest>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter_mut() {
        let client = request.0 .0.clone();
        let queue_url = request.0 .1.clone();

        info!("receiving messages from queue at {}...", queue_url);

        let task = runtime.spawn_background_task(|_ctx| async move {
            client.receive_message().queue_url(queue_url).send().await
        });

        commands
            .entity(entity)
            .insert(SQSReceiveMessageTask(task))
            .remove::<SQSReceiveMessageRequest>();
    }
}

pub fn poll_sqs_receive_message_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut SQSReceiveMessageTask)>,
) {
    for (entity, mut task) in requests.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            // TODO: error handling
            let result = result.unwrap();

            commands
                .entity(entity)
                .insert(SQSReceiveMessageResult(Some(result)))
                .remove::<SQSReceiveMessageTask>();
        }
    }
}

pub fn start_sqs_delete_message_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut SQSDeleteMessageRequest), Added<SQSDeleteMessageRequest>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter_mut() {
        let client = request.0 .0.clone();
        let queue_url = request.0 .1.clone();
        let message_reciept_handle = request.0 .2.clone();

        info!(
            "deleting message {} from queue at {}...",
            message_reciept_handle, queue_url
        );

        let task = runtime.spawn_background_task(|_ctx| async move {
            client
                .delete_message()
                .queue_url(queue_url)
                .receipt_handle(message_reciept_handle)
                .send()
                .await
        });

        commands
            .entity(entity)
            .insert(SQSDeleteMessageTask(task))
            .remove::<SQSDeleteMessageRequest>();
    }
}

pub fn poll_sqs_delete_message_requests(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut SQSDeleteMessageTask)>,
) {
    for (entity, mut task) in requests.iter_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            // TODO: error handling
            let result = result.unwrap();

            commands
                .entity(entity)
                .insert(SQSDeleteMessageResult(Some(result)))
                .remove::<SQSDeleteMessageTask>();
        }
    }
}
