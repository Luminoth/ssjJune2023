#![cfg(feature = "server")]

use bevy::prelude::*;

use crate::states::GameState;
use crate::systems::init_server::*;

pub struct InitServerPlugin;

impl Plugin for InitServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::InitServer)))
            .add_system(wait_for_aws_config_task.in_set(OnUpdate(GameState::InitServer)))
            .add_system(teardown.in_schedule(OnExit(GameState::InitServer)));
    }
}
