use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_tokio_tasks::TaskContext;
use futures_lite::FutureExt;

use crate::components::{client::main_menu::*, reqwest::*};
use crate::events::client::auth::*;
use crate::plugins::client::main_menu::*;
use crate::resources::{client::auth::*, reqwest::*};
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
    mut commands: Commands,
    mut events: EventReader<AuthenticationResult>,
    reqwest_client: Res<ReqwestClient>,
    authorization: Res<AuthorizationResource>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.label("Waiting for authentication ...");
    });

    if let Some(event) = events.iter().next() {
        if event.0 {
            debug!("authentication success");

            // TODO: error handling
            let request = reqwest_client
                .get("http://localhost:3000/user")
                .bearer_auth(authorization.get_access_token())
                .build()
                .unwrap();

            commands.spawn(ReqwestRequest((
                request,
                // TODO: this should be cleaned up
                std::sync::Arc::new(move |resp, ctx| user_response_handler(resp, ctx).boxed()),
            )));

            main_menu_state.set(MainMenuState::WaitForUser);
        } else {
            error!("authentication failure");

            main_menu_state.set(MainMenuState::WaitForLogin);
        }
    }

    events.clear();
}

async fn user_response_handler(_resp: Result<bytes::Bytes, reqwest::Error>, mut _ctx: TaskContext) {
    info!("got user response");
}

pub fn wait_for_user(mut contexts: EguiContexts, mut _game_state: ResMut<NextState<GameState>>) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.label("Retrieving user ...");
    });

    /*if let Some(event) = events.iter().next() {
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

        events.clear();*/
}
