use bevy::prelude::*;

use crate::plugins::server::working::*;
use crate::resources::server::*;
use crate::states::GameState;

use common::messages::Message;

pub fn enter(
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

pub fn exit() {
    info!("exiting Working state");
}

pub fn duel(
    mut working_state: ResMut<NextState<WorkingState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    // TODO: do stuff

    working_state.set(WorkingState::Init);
    game_state.set(GameState::LookingForWork);
}
