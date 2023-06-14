use bevy::prelude::*;

use crate::components::server::working::*;
use crate::plugins::server::working::*;
use crate::resources::server::*;
use crate::states::GameState;

use common::messages::Message;

pub fn setup(
    mut commands: Commands,
    message: Res<WorkMessage>,
    mut working_state: ResMut<NextState<WorkingState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    info!("entering Working state");

    commands.remove_resource::<WorkMessage>();

    let message: Message = (&(**message)).into();
    match message {
        Message::Invalid => {
            warn!("invalid message {:?}", message);

            game_state.set(GameState::LookingForWork);
        }
        Message::Duel(_message) => {
            info!("handling Duel message");

            // TODO: add the player and opponent entities

            working_state.set(WorkingState::Duel);
        }
    }
}

pub fn teardown(mut commands: Commands, to_despawn: Query<Entity, With<OnWorking>>) {
    info!("exiting Working state");

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn duel(
    mut working_state: ResMut<NextState<WorkingState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // TODO: do stuff

    working_state.set(WorkingState::Init);
    game_state.set(GameState::LookingForWork);
}
