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
            .add_system(enter.in_schedule(OnEnter(GameState::LookingForWork)))
            .add_systems((
                get_queue_url.in_schedule(OnEnter(LookingForWorkState::GetQueueUrl)),
                wait_for_queue_url.in_set(OnUpdate(LookingForWorkState::GetQueueUrl)),
            ))
            .add_systems((
                look_for_work.in_schedule(OnEnter(LookingForWorkState::LookForWork)),
                wait_for_work.in_set(OnUpdate(LookingForWorkState::LookForWork)),
            ))
            .add_systems((wait_for_claim_work.in_set(OnUpdate(LookingForWorkState::ClaimWork)),))
            .add_system(exit.in_schedule(OnExit(GameState::LookingForWork)))
            .add_system(
                cleanup_state::<OnLookingForWork>.in_schedule(OnExit(GameState::LookingForWork)),
            );
    }
}
