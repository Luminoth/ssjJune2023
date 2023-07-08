use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_tokio_tasks::TaskContext;
use futures_lite::future::FutureExt;
use hyper::{Body, Method, Request, Response, StatusCode};
use serde::Deserialize;

use common::http::*;

use crate::components::{client::main_menu::*, hyper::*, reqwest::*};
use crate::plugins::client::main_menu::*;
use crate::resources::client::main_menu::*;
use crate::states::GameState;

#[derive(Debug, Deserialize)]
struct AccessTokenRequest {
    pub access_token: String,
}

async fn auth_request_handler(
    port: u16,
    req: Request<Body>,
    mut ctx: TaskContext,
) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            debug!("got GET to '/': {:?}", req);

            /*
            itch puts the token in the path fragment even when using loopback
            browsers don't send that over to us so we need some javascript
            to pull it out and re-POST it to us

            TODO: is there really not a better way to do this??
            */

            Ok(Response::new(
                format!(
                    "<!DOCTYPE html>
<html lang=\"en-US\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
    <title>Success</title>
    <script>
        var queryString = window.location.hash.slice(1);
        var params = new URLSearchParams(queryString);
        var accessToken = params.get('access_token');
        fetch('http://127.0.0.1:{}', {{
            method: 'POST',
            headers: {{
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            }},
            body: JSON.stringify({{ 'access_token': accessToken }})
        }})
        .then(response => response.json())
        .then(response => console.log(JSON.stringify(response)));
    </script>
</head>
<body>
    <div>You can close this window now!</div>
</body>
</html>",
                    port
                )
                .into(),
            ))
        }
        (&Method::POST, "/") => {
            debug!("got POST to '/': {:?}", req);

            // TODO: error handling
            let body = hyper::body::to_bytes(req.into_body()).await.unwrap();
            let request: AccessTokenRequest =
                serde_json::from_slice(body.to_vec().as_slice()).unwrap();

            ctx.run_on_main_thread(move |ctx| {
                debug!("got access token: {}", request.access_token);

                ctx.world
                    .get_resource_mut::<AuthenticationToken>()
                    .unwrap()
                    .0 = request.access_token;
            })
            .await;

            Ok(Response::default())
        }
        _ => {
            debug!("http listener returning not found: {:?}", req);

            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub fn enter(mut commands: Commands, mut main_menu_state: ResMut<NextState<MainMenuState>>) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnMainMenu));

    // TODO: if we have a token saved in storage, try that first

    commands.spawn(StartHyperListener((
        5000,
        // TODO: this should be cleaned up
        std::sync::Arc::new(move |port, req, ctx| auth_request_handler(port, req, ctx).boxed()),
    )));

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
                // TODO: if we were unable to start the listener
                // we should have this redirect to 'urn:ietf:wg:oauth:2.0:oob' instead
                // and show an input prompt for the token
                webbrowser::open("https://itch.io/user/oauth?client_id=11608a8d9cd812ac0651da4dc2f9f484&scope=profile%3Ame&response_type=token&redirect_uri=http%3A%2F%2F127.0.0.1%3A5000").unwrap();

                main_menu_state.set(MainMenuState::WaitForOAuth);
            }
        });
    });
}

pub fn wait_for_oauth(
    mut commands: Commands,
    auth_token: ResMut<AuthenticationToken>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
    mut contexts: EguiContexts,
) {
    egui::Window::new("Authentication").show(contexts.ctx_mut(), |ui| {
        ui.label("Waiting for authorization ...");

        /*ui.horizontal(|ui| {
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
        });*/
    });

    if !auth_token.0.trim().is_empty() {
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

                // TODO: save off the token

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
