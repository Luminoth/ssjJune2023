use aws_config::SdkConfig;
use aws_sdk_sqs::{
    error::SdkError,
    operation::delete_message::{DeleteMessageError, DeleteMessageOutput},
    operation::get_queue_url::{GetQueueUrlError, GetQueueUrlOutput},
    operation::receive_message::{ReceiveMessageError, ReceiveMessageOutput},
};
use bevy::prelude::*;
use tokio::task;

#[derive(Debug, Component)]
pub struct LoadAwsConfigRequest;

#[derive(Debug, Component)]
pub struct LoadAwsConfigTask(pub task::JoinHandle<SdkConfig>);

#[derive(Debug, Component)]
pub struct LoadAwsConfigResult(pub Option<SdkConfig>);

#[derive(Debug, Component)]
pub struct SQSGetQueueUrlRequest(pub (aws_sdk_sqs::Client, String));

#[derive(Debug, Component)]
pub struct SQSGetQueueUrlTask(
    pub task::JoinHandle<Result<GetQueueUrlOutput, SdkError<GetQueueUrlError>>>,
);

#[derive(Debug, Component)]
pub struct SQSGetQueueUrlResult(pub Option<Result<GetQueueUrlOutput, SdkError<GetQueueUrlError>>>);

#[derive(Debug, Component)]
pub struct SQSReceiveMessageRequest(pub (aws_sdk_sqs::Client, String));

#[derive(Debug, Component)]
pub struct SQSReceiveMessageTask(
    pub task::JoinHandle<Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>>,
);

#[derive(Debug, Component)]
pub struct SQSReceiveMessageResult(
    pub Option<Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>>,
);

#[derive(Debug, Component)]
pub struct SQSDeleteMessageRequest(pub (aws_sdk_sqs::Client, String, String));

#[derive(Debug, Component)]
pub struct SQSDeleteMessageTask(
    pub task::JoinHandle<Result<DeleteMessageOutput, SdkError<DeleteMessageError>>>,
);

#[derive(Debug, Component)]
pub struct SQSDeleteMessageResult(
    pub Option<Result<DeleteMessageOutput, SdkError<DeleteMessageError>>>,
);
