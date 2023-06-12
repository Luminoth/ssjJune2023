#![cfg(feature = "server")]

use bevy::prelude::*;

use crate::states::GameState;
use crate::systems::looking_for_work::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum LookingForWorkState {
    #[default]
    Init,
    GetQueueUrl,
    LookForWork,
}

pub struct LookingForWorkPlugin;

impl Plugin for LookingForWorkPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<LookingForWorkState>()
            .add_system(setup.in_schedule(OnEnter(GameState::LookingForWork)))
            .add_systems((
                get_queue_url.in_schedule(OnEnter(LookingForWorkState::GetQueueUrl)),
                wait_for_queue_url.in_set(OnUpdate(LookingForWorkState::GetQueueUrl)),
            ))
            .add_systems((
                look_for_work.in_schedule(OnEnter(LookingForWorkState::LookForWork)),
                wait_for_work.in_set(OnUpdate(LookingForWorkState::LookForWork)),
            ))
            .add_system(teardown.in_schedule(OnExit(GameState::LookingForWork)));
    }
}
