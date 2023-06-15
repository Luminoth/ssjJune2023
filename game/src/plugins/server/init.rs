use bevy::prelude::*;

use crate::components::server::init::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, server::init::*};

pub struct InitServerPlugin;

impl Plugin for InitServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enter.in_schedule(OnEnter(GameState::InitServer)))
            .add_system(wait_for_aws_config_task.in_set(OnUpdate(GameState::InitServer)))
            .add_system(exit.in_schedule(OnExit(GameState::InitServer)))
            .add_system(cleanup_state::<OnInitServer>.in_schedule(OnExit(GameState::InitServer)));
    }
}
