use bevy::prelude::*;

use crate::cooldown::Cooldown;

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct WorkQueueUrl(pub String);

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct LookForWorkCooldown(pub Cooldown);
