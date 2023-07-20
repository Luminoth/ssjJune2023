use bevy::prelude::*;
use bevy_tokio_tasks::TaskContext;
use futures_lite::FutureExt;
use hyper::{Body, Method, Request, Response, StatusCode};
use serde::Deserialize;

use common::http::*;

use crate::components::{hyper::*, reqwest::*};
use crate::events::client::auth::*;
use crate::resources::client::auth::*;

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

fn start_oauth(commands: &mut Commands, auth_state: &mut AuthenticationState) {
    commands.spawn(StartHyperListener((
        5000,
        // TODO: this should be cleaned up
        std::sync::Arc::new(move |port, req, ctx| auth_request_handler(port, req, ctx).boxed()),
    )));

    // TODO: if we were unable to start the listener
    // we should have this redirect to 'urn:ietf:wg:oauth:2.0:oob' instead
    // and show an input prompt for the token
    webbrowser::open("https://itch.io/user/oauth?client_id=11608a8d9cd812ac0651da4dc2f9f484&scope=profile%3Ame&response_type=token&redirect_uri=http%3A%2F%2F127.0.0.1%3A5000").unwrap();

    *auth_state = AuthenticationState::WaitForAuthorization;
}

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
                    .get_resource_mut::<AuthorizationResource>()
                    .unwrap()
                    .set_oauth_token(request.access_token.clone());
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

fn authenticate(
    commands: &mut Commands,
    auth_state: &mut AuthenticationState,
    oauth_token: impl Into<String>,
) {
    let client = reqwest::Client::new();

    let request = client
        .post("http://localhost:3000/authenticate")
        .json(&AuthenticateRequest {
            oauth_token: oauth_token.into(),
        })
        .build()
        .unwrap();

    commands.spawn(ReqwestRequest((client, request)));

    *auth_state = AuthenticationState::WaitForAuthentication;
}

fn refresh(
    commands: &mut Commands,
    auth_state: &mut AuthenticationState,
    refresh_token: impl Into<String>,
) {
    let client = reqwest::Client::new();

    let request = client
        .post("http://localhost:3000/refresh")
        .json(&RefreshRequest {
            refresh_token: refresh_token.into(),
        })
        .build()
        .unwrap();

    commands.spawn(ReqwestRequest((client, request)));

    *auth_state = AuthenticationState::WaitForRefresh;
}

pub fn refresh_auth_listener(
    mut commands: Commands,
    mut events: EventReader<RefreshAuthentication>,
    mut auth_state: ResMut<AuthenticationState>,
    authorization: Res<AuthorizationResource>,
) {
    if events.is_empty() {
        return;
    }

    match *auth_state {
        AuthenticationState::Unauthorized => {
            // if we have a valid access token, we can skip authorization
            if !authorization.is_access_token_expired() {
                *auth_state = AuthenticationState::Authenticated;

                // do we need to refresh?
                if authorization.should_refresh_access_token() {
                    refresh(
                        &mut commands,
                        &mut auth_state,
                        authorization.get_refresh_token().clone(),
                    );
                }
                return;
            }

            // if we have a valid refresh token, we can skip authorization
            if !authorization.is_refresh_token_expired() {
                refresh(
                    &mut commands,
                    &mut auth_state,
                    authorization.get_refresh_token().clone(),
                );
                return;
            }

            // no valid tokens, so we need to authorize
            start_oauth(&mut commands, &mut auth_state);
        }
        AuthenticationState::Unauthenticated => {
            authenticate(
                &mut commands,
                &mut auth_state,
                authorization.get_oauth_token().clone(),
            );
        }
        AuthenticationState::Authenticated => {
            // do we need to refresh?
            if authorization.should_refresh_access_token() {
                refresh(
                    &mut commands,
                    &mut auth_state,
                    authorization.get_refresh_token(),
                );
            }
        }
        AuthenticationState::WaitForAuthorization
        | AuthenticationState::WaitForAuthentication
        | AuthenticationState::WaitForRefresh => {
            // ignore requests while we're waiting for something
        }
    }

    events.clear();
}
