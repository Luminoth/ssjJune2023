use bevy::prelude::*;

use crate::systems::hyper::*;

pub struct HyperPlugin;

impl Plugin for HyperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                start_http_listeners,
                stop_http_listeners,
                poll_http_listeners,
            ),
        );
    }
}
