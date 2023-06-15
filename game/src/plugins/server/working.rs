#![cfg(feature = "server")]

use bevy::prelude::*;

use crate::components::server::working::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, server::working::*};

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
            .add_system(enter.in_schedule(OnEnter(GameState::Working)))
            .add_systems((duel.in_set(OnUpdate(WorkingState::Duel)),))
            .add_system(exit.in_schedule(OnExit(GameState::Working)))
            .add_system(cleanup_state::<OnWorking>.in_schedule(OnExit(GameState::Working)));
    }
}
