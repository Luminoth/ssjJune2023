use bevy::prelude::*;

use crate::systems::hyper::*;

pub struct HyperPlugin;

impl Plugin for HyperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((start_http_listener, poll_http_listeners));
    }
}
