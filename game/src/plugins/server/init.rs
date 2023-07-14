use bevy::prelude::*;

use crate::components::server::init::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, server::init::*};

pub struct InitServerPlugin;

impl Plugin for InitServerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InitServer), enter)
            .add_systems(
                Update,
                wait_for_aws_config_task.run_if(in_state(GameState::InitServer)),
            )
            .add_systems(
                OnExit(GameState::InitServer),
                (exit, cleanup_state::<OnInitServer>),
            );
    }
}
