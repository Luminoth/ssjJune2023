use bevy::prelude::*;

use crate::systems::notifs::*;

pub struct NotifsPlugin;

impl Plugin for NotifsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (subscribe_notifs, poll_subscribe_notifs),
                (listen_notifs, poll_listen_notifs),
            ),
        );
    }
}
