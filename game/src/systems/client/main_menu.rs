use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::client::main_menu::*;
use crate::plugins::client::main_menu::*;
use crate::resources::client::main_menu::*;

pub fn enter(mut commands: Commands, mut main_menu_state: ResMut<NextState<MainMenuState>>) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnMainMenu));

    commands.insert_resource(AuthenticationToken(String::default()));

    main_menu_state.set(MainMenuState::WaitForLogin);
}

pub fn exit(mut commands: Commands) {
    info!("exiting MainMenu state");

    commands.remove_resource::<AuthenticationToken>();
}

pub fn wait_for_login(
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Login").clicked() {
                //webbrowser::open("https://itch.io/user/oauth?client_id=foobar&scope=profile:me&redirect_uri=urn:ietf:wg:oauth:2.0:oob").unwrap();

                main_menu_state.set(MainMenuState::WaitForOAuth);
            }
        });
    });
}

pub fn wait_for_oauth(
    mut auth_token: ResMut<AuthenticationToken>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("Enter authentication token:");
            ui.text_edit_singleline(&mut auth_token.0);
        });

        ui.horizontal(|ui| {
            ui.add_enabled_ui(!auth_token.0.trim().is_empty(), |ui| {
                if ui.button("Ok").clicked() {
                    main_menu_state.set(MainMenuState::WaitForAuth);
                }
            });

            if ui.button("Cancel").clicked() {
                auth_token.0.clear();
                main_menu_state.set(MainMenuState::WaitForLogin);
            }
        });
    });
}

pub fn wait_for_auth(
    mut _main_menu_state: ResMut<NextState<MainMenuState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |_ui| {
        // state goes back to Init
    });
}
