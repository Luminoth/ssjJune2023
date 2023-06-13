use bevy::prelude::*;

use crate::cooldown::Cooldown;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct SqsClient(pub aws_sdk_sqs::Client);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct QueueUrl(pub String);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct LookForWorkCooldown(pub Cooldown);
