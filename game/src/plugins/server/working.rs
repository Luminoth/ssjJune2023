#![cfg(feature = "server")]

use bevy::prelude::*;

use crate::states::GameState;
use crate::systems::server::working::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum WorkingState {
    #[default]
    Init,
    Duel,
}

pub struct WorkingPlugin;

impl Plugin for WorkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<WorkingState>()
            .add_system(setup.in_schedule(OnEnter(GameState::Working)))
            .add_systems((duel.in_set(OnUpdate(WorkingState::Duel)),))
            .add_system(teardown.in_schedule(OnExit(GameState::Working)));
    }
}
