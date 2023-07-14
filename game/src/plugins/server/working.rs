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
            .add_systems(OnEnter(GameState::Working), enter)
            .add_systems(Update, duel.run_if(in_state(WorkingState::Duel)))
            .add_systems(
                OnExit(GameState::Working),
                (exit, cleanup_state::<OnWorking>),
            );
    }
}
