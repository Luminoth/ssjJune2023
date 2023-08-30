use bevy::prelude::*;

use crate::events::notifs::*;
use crate::systems::notifs::*;

pub struct NotifsPlugin;

impl Plugin for NotifsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NotifsSubscribeResult>()
            .add_event::<NotifsDisconnected>()
            .add_event::<Notification>()
            .add_systems(
                Update,
                (
                    (subscribe_notifs, poll_subscribe_notifs),
                    (listen_notifs, poll_listen_notifs),
                ),
            );
    }
}
