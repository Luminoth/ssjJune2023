use bevy::prelude::*;

use crate::systems::client::reqwest::*;

pub struct ReqwestPlugin;

impl Plugin for ReqwestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((start_http_requests, poll_http_requests));
    }
}
