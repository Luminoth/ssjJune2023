use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
//use bevy_persistent::prelude::*;

use common::http::*;

use crate::auth::{self, AuthorizationResource};
use crate::components::{client::main_menu::*, reqwest::*};
use crate::plugins::client::main_menu::*;
use crate::states::GameState;

pub fn enter(
    mut commands: Commands,
    auth_token: Res<AuthorizationResource>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnMainMenu));

    if !auth_token.access_token.trim().is_empty() {
        // TODO: this isn't right, we want to refresh our token
        // or otherwise do something to verify the token first
        game_state.set(GameState::Game);
    } else {
        main_menu_state.set(MainMenuState::WaitForLogin);
    }
}

pub fn exit(mut commands: Commands) {
    info!("exiting MainMenu state");

    auth::cleanup(&mut commands);
}

pub fn wait_for_login(
    mut commands: Commands,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Login").clicked() {
                auth::start_oauth(&mut commands);

                main_menu_state.set(MainMenuState::WaitForOAuth);
            }
        });
    });
}

pub fn wait_for_oauth(
    mut commands: Commands,
    auth_token: Res<AuthorizationResource>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.label("Waiting for authorization ...");

        // TODO: if the oauth listener failed
        // show this UI to enter the token
        /*ui.horizontal(|ui| {
            ui.label("Enter authentication token:");
            ui.text_edit_singleline(&mut auth_token.access_token);
        });

        ui.horizontal(|ui| {
            ui.add_enabled_ui(!auth_token.access_token.trim().is_empty(), |ui| {
                if ui.button("Ok").clicked() {
                    auth::authenticate(&mut commands, auth_token.oauth_token.clone());

                    main_menu_state.set(MainMenuState::WaitForAuth);
                }
            });

            if ui.button("Cancel").clicked() {
                auth_token.access_token.clear();
                main_menu_state.set(MainMenuState::WaitForLogin);
            }
        });*/
    });

    if !auth_token.oauth_token.trim().is_empty() {
        auth::authenticate(&mut commands, auth_token.oauth_token.clone());

        main_menu_state.set(MainMenuState::WaitForAuth);
    }
}

pub fn wait_for_auth(
    mut commands: Commands,
    mut results: Query<(Entity, &mut ReqwestResult)>,
    mut auth_token: ResMut<AuthorizationResource>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.label("Waiting for authentication ...");
    });

    if let Ok((entity, mut result)) = results.get_single_mut() {
        // TODO: error handling
        let result = result.0.take().unwrap();

        match result {
            Ok(response) => {
                info!("authentication success");

                // TODO: error handling
                let response = serde_json::from_slice::<AuthenticateResponse>(&response).unwrap();
                auth_token
                    /*.update(|auth| {
                        auth.access_token = response.token.clone();
                    });*/
                    .access_token = response.access_token.clone();

                game_state.set(GameState::Game);

                main_menu_state.set(MainMenuState::Init);
            }
            Err(err) => {
                error!("http error: {:?}", err);

                main_menu_state.set(MainMenuState::WaitForLogin);
            }
        }

        auth_token.oauth_token.clear();

        commands.entity(entity).despawn_recursive();
    }
}
