use bevy::prelude::*;

use crate::events::notifs::*;
use crate::resources::client::auth::*;
use crate::states::GameState;

pub fn notifications_subscribe_handler(
    mut events: EventReader<NotifsSubscribeResult>,
    mut auth_error: ResMut<AuthenticationError>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for event in events.iter() {
        if !event.0 .1 {
            auth_error.0 = Some("Connection error".to_owned());

            game_state.set(GameState::MainMenu);
        }
    }
}

pub fn notifications_disconnected_handler(
    mut events: EventReader<NotifsDisconnected>,
    mut auth_error: ResMut<AuthenticationError>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if !events.is_empty() {
        auth_error.0 = Some("Disconnected".to_owned());

        game_state.set(GameState::MainMenu);
    }

    events.clear();
}

pub fn notification_handler(mut events: EventReader<Notification>) {
    for event in events.iter() {
        let uri = &event.0 .0;
        let message = &event.0 .1;

        // TODO:
        info!("got notification from {}: {}", uri, message);
    }
}
