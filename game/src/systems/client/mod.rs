#![cfg(feature = "client")]

pub mod main_menu;
pub mod splash;

use bevy::prelude::*;

use crate::auth::AuthorizationResource;

pub fn startup(mut commands: Commands) {
    let _config_dir = dirs::config_dir()
        .map(|native_config_dir| native_config_dir.join("ssj2023"))
        .unwrap_or(std::path::Path::new("local").join("configuration"));
    commands.insert_resource(
        /*Persistent::<Authorization>::builder()
        .name("authorization")
        .format(StorageFormat::Ini)
        .path(config_dir.join("authorization.ini"))
        .default(Authorization::default())
        .build(),*/
        AuthorizationResource::default(),
    );
}
