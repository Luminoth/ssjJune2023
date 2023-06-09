#![cfg(feature = "server")]

pub mod looking_for_work;

use aws_config::SdkConfig;
use bevy::prelude::*;

use crate::cooldown::Throttle;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct AwsConfig(pub SdkConfig);

#[derive(Debug, Resource, Default, Deref, DerefMut)]
pub struct AwsThrottle(pub Throttle);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct SqsClient(pub aws_sdk_sqs::Client);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct WorkMessage(pub String);
