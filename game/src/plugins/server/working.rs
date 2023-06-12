#![cfg(feature = "server")]

use bevy::prelude::*;

use crate::states::GameState;
use crate::systems::server::working::*;

pub struct WorkingPlugin;

impl Plugin for WorkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Working)))
            .add_system(teardown.in_schedule(OnExit(GameState::Working)));
    }
}
