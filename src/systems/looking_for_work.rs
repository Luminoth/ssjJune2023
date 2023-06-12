#![cfg(feature = "server")]

use bevy::prelude::*;
use bevy_tokio_tasks::*;
use futures_lite::future;

use crate::components::looking_for_work::*;
use crate::resources::{looking_for_work::*, server::*};
use crate::states::GameState;

pub fn setup(
    mut commands: Commands,
    aws_config: Res<AwsConfig>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    info!("entering LookingForWork state");

    let client = aws_sdk_sqs::Client::new(&aws_config);

    let task = runtime.spawn_background_task({
        let client = client.clone();
        |_ctx| async move {
            info!("getting queue URL...");
            client.get_queue_url().queue_name("ssj2023").send().await
        }
    });

    commands.spawn((QueueUrlTask(task), OnLookingForWork));

    commands.insert_resource(SqsClient(client));
}

pub fn teardown(mut commands: Commands, to_despawn: Query<Entity, With<OnLookingForWork>>) {
    info!("exiting LookingForWork state");

    commands.remove_resource::<QueueUrl>();
    commands.remove_resource::<SqsClient>();

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn wait_for_queue_url(
    mut commands: Commands,
    mut queue_url_tasks: Query<(Entity, &mut QueueUrlTask)>,
) {
    if let Ok((entity, mut task)) = queue_url_tasks.get_single_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            // TODO: error handling
            let result = result.unwrap();

            match result {
                Ok(output) => {
                    // TODO: error handling
                    let queue_url = output.queue_url().unwrap().to_owned();
                    info!("queue url: {}", queue_url);

                    commands.insert_resource(QueueUrl(queue_url));
                }
                Err(err) => {
                    info!("queue url error: {:?}", err);

                    // TODO: throttle
                }
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn look_for_work(
    mut commands: Commands,
    receive_message_tasks: Query<(Entity, &ReceiveMessageTask)>,
    client: Res<SqsClient>,
    queue_url: Option<Res<QueueUrl>>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    if receive_message_tasks.get_single().is_ok() {
        return;
    }

    if let Some(queue_url) = queue_url {
        let client = client.0.clone();
        let queue_url = queue_url.0.clone();
        let task = runtime.spawn_background_task(|_ctx| async move {
            info!("checking for work...");
            client.receive_message().queue_url(queue_url).send().await
        });

        commands.spawn((ReceiveMessageTask(task), OnLookingForWork));
    }
}

pub fn wait_for_work(
    mut commands: Commands,
    mut receive_message_tasks: Query<(Entity, &mut ReceiveMessageTask)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, mut task)) = receive_message_tasks.get_single_mut() {
        if let Some(result) = future::block_on(future::poll_once(&mut task.0)) {
            // TODO: error handling
            let result = result.unwrap();

            match result {
                Ok(output) => {
                    if let Some(messages) = output.messages() {
                        info!("messages: {:?}", messages);

                        // TODO: do something with the messages

                        // TODO: have to delete the message from the queue

                        game_state.set(GameState::Working);
                    } else {
                        info!("no work");

                        // TODO: throttle
                    }
                }
                Err(err) => {
                    info!("error: {:?}", err);

                    // TODO: throttle
                }
            }

            commands.entity(entity).despawn_recursive();
        }
    }
}
