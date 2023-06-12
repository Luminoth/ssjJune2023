#![cfg(feature = "server")]

use bevy::prelude::*;

#[derive(Resource, Deref, DerefMut)]
pub struct SqsClient(pub aws_sdk_sqs::Client);

#[derive(Resource, Deref, DerefMut)]
pub struct QueueUrl(pub String);
