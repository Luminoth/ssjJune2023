use bevy::prelude::*;

use crate::resources::reqwest::*;
use crate::systems::reqwest::*;

pub struct ReqwestPlugin;

impl Plugin for ReqwestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ReqwestClient::default())
            .add_systems(Update, (start_http_requests, poll_http_requests));
    }
}
