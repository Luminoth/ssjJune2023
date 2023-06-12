use aws_sdk_sqs::{
    error::SdkError,
    operation::get_queue_url::{GetQueueUrlError, GetQueueUrlOutput},
    operation::receive_message::{ReceiveMessageError, ReceiveMessageOutput},
};
use bevy::prelude::*;
use tokio::task;

#[derive(Component)]
pub struct OnLookingForWork;

#[derive(Component)]
pub struct QueueUrlTask(
    pub task::JoinHandle<Result<GetQueueUrlOutput, SdkError<GetQueueUrlError>>>,
);

#[derive(Component)]
pub struct ReceiveMessageTask(
    pub task::JoinHandle<Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>>,
);
