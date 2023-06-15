use bevy::prelude::*;

use crate::components::server::{aws::*, looking_for_work::*};
use crate::cooldown::Cooldown;
use crate::plugins::server::looking_for_work::*;
use crate::resources::{server::looking_for_work::*, server::*, Random};
use crate::states::GameState;

pub fn enter(
    mut commands: Commands,
    aws_config: Res<AwsConfig>,
    mut looking_for_work_state: ResMut<NextState<LookingForWorkState>>,
) {
    info!("entering LookingForWork state");

    let client = aws_sdk_sqs::Client::new(&aws_config);
    commands.insert_resource(SqsClient(client));

    commands.insert_resource(AwsThrottle::default());
    commands.insert_resource(LookForWorkCooldown(Cooldown::new(
        bevy::utils::Duration::from_secs(5),
    )));

    looking_for_work_state.set(LookingForWorkState::GetQueueUrl);
}

pub fn exit(mut commands: Commands) {
    info!("exiting LookingForWork state");

    commands.remove_resource::<AwsThrottle>();
    commands.remove_resource::<LookForWorkCooldown>();
    commands.remove_resource::<WorkQueueUrl>();
    commands.remove_resource::<SqsClient>();
}

pub fn get_queue_url(
    mut commands: Commands,
    client: Res<SqsClient>,
    mut throttle: ResMut<AwsThrottle>,
    mut looking_for_work_state: ResMut<NextState<LookingForWorkState>>,
    time: Res<Time>,
) {
    throttle.tick(time.delta());

    if !throttle.finished() {
        debug!("waiting for throttle");
        looking_for_work_state.set(LookingForWorkState::GetQueueUrl);
        return;
    }

    commands.spawn((
        SQSGetQueueUrlRequest((client.clone(), "ssj2023".to_owned())),
        OnLookingForWork,
    ));
}

pub fn wait_for_queue_url(
    mut commands: Commands,
    mut results: Query<(Entity, &mut SQSGetQueueUrlResult)>,
    mut throttle: ResMut<AwsThrottle>,
    mut random: ResMut<Random>,
    mut looking_for_work_state: ResMut<NextState<LookingForWorkState>>,
) {
    if let Ok((entity, mut result)) = results.get_single_mut() {
        // TODO: error handling
        let result = result.0.take().unwrap();

        match result {
            Ok(output) => {
                // TODO: error handling
                let queue_url = output.queue_url().unwrap().to_owned();
                info!("queue url: {}", queue_url);

                throttle.reset();
                commands.insert_resource(WorkQueueUrl(queue_url));

                looking_for_work_state.set(LookingForWorkState::LookForWork);
            }
            Err(err) => {
                error!("queue url error: {:?}", err);

                throttle.start(&mut random);

                looking_for_work_state.set(LookingForWorkState::GetQueueUrl);
            }
        }

        commands.entity(entity).despawn_recursive();
    }
}

#[allow(clippy::too_many_arguments)]
pub fn look_for_work(
    mut commands: Commands,
    client: Res<SqsClient>,
    queue_url: Res<WorkQueueUrl>,
    mut throttle: ResMut<AwsThrottle>,
    mut cooldown: ResMut<LookForWorkCooldown>,
    mut looking_for_work_state: ResMut<NextState<LookingForWorkState>>,
    time: Res<Time>,
) {
    throttle.tick(time.delta());
    cooldown.tick(time.delta());

    if !throttle.finished() {
        debug!("waiting for throttle");
        looking_for_work_state.set(LookingForWorkState::LookForWork);
        return;
    }

    if !cooldown.finished() {
        debug!("waiting for cooldown");
        looking_for_work_state.set(LookingForWorkState::LookForWork);
        return;
    }

    commands.spawn((
        SQSReceiveMessageRequest((client.clone(), queue_url.clone())),
        OnLookingForWork,
    ));
}

#[allow(clippy::too_many_arguments)]
pub fn wait_for_work(
    mut commands: Commands,
    client: Res<SqsClient>,
    queue_url: Res<WorkQueueUrl>,
    mut receive_message_tasks: Query<(Entity, &mut SQSReceiveMessageResult)>,
    mut throttle: ResMut<AwsThrottle>,
    mut cooldown: ResMut<LookForWorkCooldown>,
    mut random: ResMut<Random>,
    mut looking_for_work_state: ResMut<NextState<LookingForWorkState>>,
) {
    if let Ok((entity, mut result)) = receive_message_tasks.get_single_mut() {
        // TODO: error handling
        let result = result.0.take().unwrap();

        match result {
            Ok(output) => {
                if let Some(messages) = output.messages() {
                    info!("found work: {:?}", messages);
                    if messages.is_empty() {
                        warn!("unexpected no messages");

                        cooldown.start();

                        looking_for_work_state.set(LookingForWorkState::LookForWork);
                    } else if messages.len() > 1 {
                        warn!("received too many messages");
                    }

                    let message = &messages[0];

                    let message_body = message.body().unwrap().to_owned();
                    commands.insert_resource(WorkMessage(message_body));

                    let message_reciept_handle = message.receipt_handle().unwrap().to_owned();

                    commands.spawn((
                        SQSDeleteMessageRequest((
                            client.clone(),
                            queue_url.clone(),
                            message_reciept_handle.clone(),
                        )),
                        OnLookingForWork,
                    ));

                    throttle.reset();

                    looking_for_work_state.set(LookingForWorkState::ClaimWork);
                } else {
                    info!("no work");

                    cooldown.start();

                    looking_for_work_state.set(LookingForWorkState::LookForWork);
                }
            }
            Err(err) => {
                error!("receive message error: {:?}", err);

                throttle.start(&mut random);
            }
        }

        commands.entity(entity).despawn_recursive();
    }
}

pub fn wait_for_claim_work(
    mut commands: Commands,
    mut results: Query<(Entity, &mut SQSDeleteMessageResult)>,
    message: Res<WorkMessage>,
    mut throttle: ResMut<AwsThrottle>,
    mut random: ResMut<Random>,
    mut looking_for_work_state: ResMut<NextState<LookingForWorkState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, mut result)) = results.get_single_mut() {
        // TODO: error handling
        let result = result.0.take().unwrap();

        match result {
            Ok(_) => {
                info!("claimed work: {:?}", message);

                throttle.reset();

                game_state.set(GameState::Working);
                looking_for_work_state.set(LookingForWorkState::Init);
            }
            Err(err) => {
                error!("delete message error: {:?}", err);

                commands.remove_resource::<WorkMessage>();

                throttle.start(&mut random);

                looking_for_work_state.set(LookingForWorkState::LookForWork);
            }
        }

        commands.entity(entity).despawn_recursive();
    }
}
