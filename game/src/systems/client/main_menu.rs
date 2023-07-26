use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::client::main_menu::*;
use crate::events::client::auth::*;
use crate::plugins::client::main_menu::*;
use crate::resources::client::auth::*;
use crate::states::GameState;

pub fn enter(mut commands: Commands, mut main_menu_state: ResMut<NextState<MainMenuState>>) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnMainMenu));

    main_menu_state.set(MainMenuState::WaitForLogin);
}

pub fn exit() {
    info!("exiting MainMenu state");
}

pub fn wait_for_login(
    mut auth_events: EventWriter<RefreshAuthentication>,
    authorization: Res<AuthorizationResource>,
    auth_error: Res<AuthenticationError>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut contexts: EguiContexts,
) {
    // don't wait for the login button if we're already authenticated
    if auth_error.0.is_none()
        && (authorization.has_oauth() || !authorization.is_access_token_expired())
    {
        auth_events.send(RefreshAuthentication);

        main_menu_state.set(MainMenuState::WaitForAuth);

        return;
    }

    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            if ui.button("Login").clicked() {
                auth_events.send(RefreshAuthentication);

                main_menu_state.set(MainMenuState::WaitForAuth);
            }

            if let Some(auth_error) = &auth_error.0 {
                ui.label(
                    egui::RichText::new(format!("Authentication Error: {}", auth_error))
                        .color(egui::Color32::RED),
                );
            }
        });
    });
}

pub fn wait_for_auth(
    mut events: EventReader<AuthenticationResult>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.label("Waiting for authentication ...");
    });

    if let Some(event) = events.iter().next() {
        if event.0 {
            info!("authentication success");

            game_state.set(GameState::Game);

            // TODO: after auth success we need to get our user

            main_menu_state.set(MainMenuState::Init);
        } else {
            error!("authentication failure");

            main_menu_state.set(MainMenuState::WaitForLogin);
        }
    }

    events.clear();
}
