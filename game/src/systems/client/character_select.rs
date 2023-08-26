use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::client::character_select::*;

pub fn enter(mut commands: Commands) {
    info!("entering CharacterSelect state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnCharacterSelect));
}

pub fn exit() {
    info!("exiting CharacterSelect state");
}

pub fn temp_update(mut contexts: EguiContexts) {
    // TODO: need to read characters and then display them
    // also need to allow creating a new character (with a max that we get from the backend)

    egui::Window::new("Character Select").show(contexts.ctx_mut(), |_ui| {});
}
