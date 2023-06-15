use bevy::prelude::*;

use crate::components::client::main_menu::*;

pub fn enter(mut _commands: Commands, _asset_server: Res<AssetServer>) {
    info!("entering MainMenu state");
}

pub fn exit() {
    info!("exiting MainMenu state");
}
