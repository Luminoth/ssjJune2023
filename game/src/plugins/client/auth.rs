use bevy::prelude::*;

use crate::events::client::auth::*;
use crate::resources::client::auth::*;
use crate::systems::client::auth::*;

pub struct AuthPlugin;

impl Plugin for AuthPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AuthenticationState>()
            .register_type::<AuthenticationState>()
            .init_resource::<AuthenticationError>()
            .register_type::<Authorization>()
            .add_event::<RefreshAuthentication>()
            .add_event::<AuthenticationResult>()
            .add_systems(Startup, startup)
            .add_systems(Update, (refresh_auth_listener,));
    }
}
