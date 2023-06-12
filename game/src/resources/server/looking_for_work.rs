use bevy::prelude::*;

use crate::cooldown::Cooldown;

#[derive(Resource, Deref, DerefMut)]
pub struct SqsClient(pub aws_sdk_sqs::Client);

#[derive(Resource, Deref, DerefMut)]
pub struct QueueUrl(pub String);

#[derive(Resource, Deref, DerefMut)]
pub struct LookForWorkCooldown(pub Cooldown);
