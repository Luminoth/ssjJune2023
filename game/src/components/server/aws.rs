use aws_config::SdkConfig;
use aws_sdk_sqs::{
    error::SdkError,
    operation::delete_message::{DeleteMessageError, DeleteMessageOutput},
    operation::get_queue_url::{GetQueueUrlError, GetQueueUrlOutput},
    operation::receive_message::{ReceiveMessageError, ReceiveMessageOutput},
};
use bevy::prelude::*;
use tokio::task;

#[async_trait::async_trait]
pub trait AwsTaskRequest: Component + Clone {
    type Output: Send;

    async fn run(&self) -> Self::Output;
}

#[derive(Debug, Component)]
pub struct AwsTask<T> {
    task: task::JoinHandle<T>,
}

impl<T> AwsTask<T> {
    pub fn new(task: task::JoinHandle<T>) -> Self {
        Self { task }
    }

    pub fn get_handle_mut(&mut self) -> &mut task::JoinHandle<T> {
        &mut self.task
    }
}

#[derive(Debug, Component)]
pub struct AwsTaskResult<T> {
    result: Option<T>,
}

impl<T> AwsTaskResult<T> {
    pub fn new(result: Option<T>) -> Self {
        Self { result }
    }

    pub fn get_result_mut(&mut self) -> &mut Option<T> {
        &mut self.result
    }
}

#[derive(Debug, Component, Clone)]
pub struct LoadAwsConfigRequest;

#[async_trait::async_trait]
impl AwsTaskRequest for LoadAwsConfigRequest {
    type Output = SdkConfig;

    async fn run(&self) -> Self::Output {
        info!("loading AWS config...");

        aws_config::load_from_env().await
    }
}

pub type LoadAwsConfigResult = AwsTaskResult<<LoadAwsConfigRequest as AwsTaskRequest>::Output>;

#[derive(Debug, Component, Clone)]
pub struct SQSGetQueueUrlRequest(pub (aws_sdk_sqs::Client, String));

#[async_trait::async_trait]
impl AwsTaskRequest for SQSGetQueueUrlRequest {
    type Output = Result<GetQueueUrlOutput, SdkError<GetQueueUrlError>>;

    async fn run(&self) -> Self::Output {
        let client = &self.0 .0;
        let queue_name = &self.0 .1;

        info!("getting SQS queue {} URL...", queue_name);

        client.get_queue_url().queue_name(queue_name).send().await
    }
}

pub type SQSGetQueueUrlResult = AwsTaskResult<<SQSGetQueueUrlRequest as AwsTaskRequest>::Output>;

#[derive(Debug, Component, Clone)]
pub struct SQSReceiveMessageRequest(pub (aws_sdk_sqs::Client, String));

#[async_trait::async_trait]
impl AwsTaskRequest for SQSReceiveMessageRequest {
    type Output = Result<ReceiveMessageOutput, SdkError<ReceiveMessageError>>;

    async fn run(&self) -> Self::Output {
        let client = &self.0 .0;
        let queue_url = &self.0 .1;

        info!("receiving messages from queue at {}...", queue_url);

        client.receive_message().queue_url(queue_url).send().await
    }
}

pub type SQSReceiveMessageResult =
    AwsTaskResult<<SQSReceiveMessageRequest as AwsTaskRequest>::Output>;

#[derive(Debug, Component, Clone)]
pub struct SQSDeleteMessageRequest(pub (aws_sdk_sqs::Client, String, String));

#[async_trait::async_trait]
impl AwsTaskRequest for SQSDeleteMessageRequest {
    type Output = Result<DeleteMessageOutput, SdkError<DeleteMessageError>>;

    async fn run(&self) -> Self::Output {
        let client = &self.0 .0;
        let queue_url = &self.0 .1;
        let message_reciept_handle = &self.0 .2;

        info!(
            "deleting message {} from queue at {}...",
            message_reciept_handle, queue_url
        );

        client
            .delete_message()
            .queue_url(queue_url)
            .receipt_handle(message_reciept_handle)
            .send()
            .await
    }
}

pub type SQSDeleteMessageResult =
    AwsTaskResult<<SQSDeleteMessageRequest as AwsTaskRequest>::Output>;
