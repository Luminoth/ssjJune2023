use bevy::prelude::*;

use crate::components::server::looking_for_work::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, server::looking_for_work::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LookingForWorkState {
    #[default]
    Init,
    GetQueueUrl,
    LookForWork,
    ClaimWork,
}

pub struct LookingForWorkPlugin;

impl Plugin for LookingForWorkPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LookingForWorkState>()
            .add_systems(OnEnter(GameState::LookingForWork), enter)
            .add_systems(OnEnter(LookingForWorkState::GetQueueUrl), get_queue_url)
            .add_systems(OnEnter(LookingForWorkState::LookForWork), look_for_work)
            .add_systems(
                Update,
                (
                    wait_for_queue_url.run_if(in_state(LookingForWorkState::GetQueueUrl)),
                    wait_for_work.run_if(in_state(LookingForWorkState::LookForWork)),
                    wait_for_claim_work.run_if(in_state(LookingForWorkState::ClaimWork)),
                ),
            )
            .add_systems(
                OnExit(GameState::LookingForWork),
                (exit, cleanup_state::<OnLookingForWork>),
            );
    }
}
