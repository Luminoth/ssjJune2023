use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use common::http::*;

use crate::components::{client::main_menu::*, hyper::*, reqwest::*};
use crate::plugins::client::main_menu::*;
use crate::resources::client::main_menu::*;
use crate::states::GameState;

pub fn enter(mut commands: Commands, mut main_menu_state: ResMut<NextState<MainMenuState>>) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnMainMenu));

    commands.spawn(StartHyperListener(5000));

    commands.insert_resource(AuthenticationToken(String::default()));

    main_menu_state.set(MainMenuState::WaitForLogin);
}

pub fn exit(mut commands: Commands) {
    info!("exiting MainMenu state");

    commands.spawn(StopHyperListener(5000));

    commands.remove_resource::<AuthenticationToken>();
}

pub fn wait_for_login(
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Login").clicked() {
                webbrowser::open("https://itch.io/user/oauth?client_id=11608a8d9cd812ac0651da4dc2f9f484&scope=profile%3Ame&response_type=token&redirect_uri=http%3A%2F%2F127.0.0.1%3A5000").unwrap();

                main_menu_state.set(MainMenuState::WaitForOAuth);
            }
        });
    });
}

pub fn wait_for_oauth(
    mut commands: Commands,
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
                    let client = reqwest::Client::new();

                    let request = client
                        .post("http://localhost:3000/authenticate")
                        .json(&AuthenticateRequest {
                            access_token: auth_token.0.clone(),
                        })
                        .build()
                        .unwrap();

                    commands.spawn(ReqwestRequest((client, request)));

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
    mut commands: Commands,
    mut results: Query<(Entity, &mut ReqwestResult)>,
    mut auth_token: ResMut<AuthenticationToken>,
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
                info!("success: {:?}", response);

                // TODO: error handling
                let response = serde_json::from_slice::<AuthenticateResponse>(&response).unwrap();
                info!("got token {} for {}", response.token, response.display_name);

                // TODO: save off the response details

                game_state.set(GameState::Game);

                main_menu_state.set(MainMenuState::Init);
            }
            Err(err) => {
                error!("http error: {:?}", err);

                main_menu_state.set(MainMenuState::WaitForLogin);
            }
        }

        auth_token.0.clear();

        commands.entity(entity).despawn_recursive();
    }
}
