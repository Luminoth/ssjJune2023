use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_tokio_tasks::TaskContext;
use futures_lite::FutureExt;

use common::http::*;

use crate::backend;
use crate::components::{client::main_menu::*, reqwest::*};
use crate::events::client::{auth::*, *};
use crate::plugins::client::main_menu::*;
use crate::resources::{client::auth::*, client::*, reqwest::*};
use crate::states::GameState;

pub fn enter(mut commands: Commands, mut main_menu_state: ResMut<NextState<MainMenuState>>) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnMainMenu));

    main_menu_state.set(MainMenuState::WaitForLogin);
}

pub fn exit(mut main_menu_state: ResMut<NextState<MainMenuState>>) {
    info!("exiting MainMenu state");

    main_menu_state.set(MainMenuState::Init);
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
        info!("authorized, authenticating ...");

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

            backend::subscribe(&mut commands, authorization.get_access_token());

            // TODO: move this to a User plugin, similar to the Auth plugin

            // TODO: error handling
            let request = reqwest_client
                .get("http://localhost:3000/user")
                .bearer_auth(authorization.get_access_token())
                .build()
                .unwrap();

            commands.spawn(ReqwestRequest::new(request, move |resp, ctx| {
                user_response_handler(resp, ctx).boxed()
            }));

            main_menu_state.set(MainMenuState::WaitForUser);
        } else {
            error!("authentication failure");

            main_menu_state.set(MainMenuState::WaitForLogin);
        }
    }

    events.clear();
}

async fn user_response_handler(resp: Result<bytes::Bytes, reqwest::Error>, mut ctx: TaskContext) {
    ctx.run_on_main_thread(move |ctx| {
        match resp {
            Ok(response) => {
                // TODO: error handling
                let response = serde_json::from_slice::<GetUserResponse>(&response).unwrap();

                ctx.world
                    .insert_resource(User::new(response.user_id, response.display_name));

                ctx.world.send_event(UserUpdated);
            }
            Err(err) => {
                error!("http error: {:?}", err);

                // TODO: deeply error check this,

                *ctx.world.get_resource_mut::<AuthenticationState>().unwrap() =
                    AuthenticationState::Unauthorized;

                ctx.world
                    .get_resource_mut::<AuthenticationError>()
                    .unwrap()
                    .0 = Some("Connection error".to_owned());

                // TODO: this isn't right
                ctx.world.send_event(AuthenticationResult(false));
            }
        }
    })
    .await
}

pub fn wait_for_user(
    mut events: EventReader<UserUpdated>,
    user: Res<User>,
    mut game_state: ResMut<NextState<GameState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.label("Retrieving user ...");
    });

    if !events.is_empty() {
        info!("retrieved user: {:?}", user);

        game_state.set(GameState::CharacterSelect);
    }

    events.clear();
}
