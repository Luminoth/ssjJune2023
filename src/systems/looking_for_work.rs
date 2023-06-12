#![cfg(feature = "server")]

use bevy::prelude::*;
use bevy_tokio_tasks::*;
use futures_lite::future;

use crate::components::looking_for_work::*;
use crate::resources::{looking_for_work::*, server::*};
use crate::states::GameState;

pub fn setup(mut commands: Commands, aws_config: Res<AwsConfig>) {
    info!("entering LookingForWork state");

    commands.insert_resource(SqsClient(aws_sdk_sqs::Client::new(&aws_config)));
}

pub fn teardown(mut commands: Commands, to_despawn: Query<Entity, With<OnLookingForWork>>) {
    info!("exiting LookingForWork state");

    commands.remove_resource::<SqsClient>();

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn look_for_work(
    mut commands: Commands,
    receive_message_tasks: Query<(Entity, &ReceiveMessageTask)>,
    client: Res<SqsClient>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    if receive_message_tasks.get_single().is_ok() {
        return;
    }

    let client = client.0.clone();
    let task = runtime.spawn_background_task(|_ctx| async move {
        info!("waiting for work...");
        client.receive_message().queue_url("...").send().await
    });

    commands.spawn((ReceiveMessageTask(task), OnLookingForWork));
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
                    info!("result: {:?}", output);

                    // TODO: do something with the output

                    game_state.set(GameState::Working);
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
