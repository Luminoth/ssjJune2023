#![cfg(feature = "client")]

use bevy::prelude::*;
//use bevy_persistent::prelude::*;

use crate::components::hyper::*;

pub fn cleanup(commands: &mut Commands) {
    commands.spawn(StopHyperListener(5000));
}
