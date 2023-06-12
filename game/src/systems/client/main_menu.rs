use bevy::prelude::*;

use crate::components::client::main_menu::*;

pub fn setup(mut _commands: Commands, _asset_server: Res<AssetServer>) {
    info!("entering MainMenu state");
}

pub fn teardown(mut commands: Commands, to_despawn: Query<Entity, With<OnMainMenu>>) {
    info!("exiting MainMenu state");

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
