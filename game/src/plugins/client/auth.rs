use bevy::prelude::*;

use crate::events::client::auth::*;
use crate::resources::client::auth::*;
use crate::systems::client::auth::*;

pub struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AuthenticationState::Unauthorized)
            .add_event::<RefreshAuthentication>()
            .add_event::<AuthenticationResult>()
            .add_systems(Startup, startup)
            .add_systems(Update, (auth_result_listener, refresh_auth_listener));
    }
}
