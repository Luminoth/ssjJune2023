#![cfg(feature = "server")]

use aws_sdk_sqs::{
    error::SdkError,
    operation::receive_message::{ReceiveMessageError, ReceiveMessageOutput},
};
use bevy::prelude::*;
use tokio::task;

#[derive(Component)]
pub struct OnLookingForWork;

#[derive(Component)]
pub struct ReceiveMessageTask(
    pub task::JoinHandle<Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>>,
);
