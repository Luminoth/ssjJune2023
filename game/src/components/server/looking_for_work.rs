use aws_sdk_sqs::{
    error::SdkError,
    operation::delete_message::{DeleteMessageError, DeleteMessageOutput},
    operation::get_queue_url::{GetQueueUrlError, GetQueueUrlOutput},
    operation::receive_message::{ReceiveMessageError, ReceiveMessageOutput},
};
use bevy::prelude::*;
use tokio::task;

#[derive(Debug, Component)]
pub struct OnLookingForWork;

#[derive(Debug, Component)]
pub struct QueueUrlTask(
    pub task::JoinHandle<Result<GetQueueUrlOutput, SdkError<GetQueueUrlError>>>,
);

#[derive(Debug, Component)]
pub struct ReceiveMessageTask(
    pub task::JoinHandle<Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>>,
);

#[derive(Debug, Component)]
pub struct ClaimWorkTask(
    pub task::JoinHandle<Result<DeleteMessageOutput, SdkError<DeleteMessageError>>>,
);
