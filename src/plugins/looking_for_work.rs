#![cfg(feature = "server")]

use bevy::prelude::*;

use crate::states::GameState;
use crate::systems::looking_for_work::*;

pub struct LookingForWorkPlugin;

impl Plugin for LookingForWorkPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::LookingForWork)))
            .add_system(look_for_work.in_set(OnUpdate(GameState::LookingForWork)))
            .add_system(wait_for_work.in_set(OnUpdate(GameState::LookingForWork)))
            .add_system(teardown.in_schedule(OnExit(GameState::LookingForWork)));
    }
}
