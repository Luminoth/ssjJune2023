#![cfg(feature = "client")]

use bevy::prelude::*;
//use bevy_persistent::prelude::*;
use bevy_tokio_tasks::TaskContext;
use futures_lite::FutureExt;
use hyper::{Body, Method, Request, Response, StatusCode};
use serde::Deserialize;

use common::http::*;

use crate::components::{hyper::*, reqwest::*};
use crate::resources::client::Authorization;

pub type AuthorizationResource = Authorization; //Persistent<Authorization>;

#[derive(Debug, Deserialize)]
struct AccessTokenRequest {
    pub access_token: String,
}

pub fn start_oauth(commands: &mut Commands) {
    commands.spawn(StartHyperListener((
        5000,
        // TODO: this should be cleaned up
        std::sync::Arc::new(move |port, req, ctx| auth_request_handler(port, req, ctx).boxed()),
    )));

    // TODO: if we were unable to start the listener
    // we should have this redirect to 'urn:ietf:wg:oauth:2.0:oob' instead
    // and show an input prompt for the token
    webbrowser::open("https://itch.io/user/oauth?client_id=11608a8d9cd812ac0651da4dc2f9f484&scope=profile%3Ame&response_type=token&redirect_uri=http%3A%2F%2F127.0.0.1%3A5000").unwrap();
}

pub fn authenticate(commands: &mut Commands, access_token: impl Into<String>) {
    let client = reqwest::Client::new();

    let request = client
        .post("http://localhost:3000/authenticate")
        .json(&AuthenticateRequest {
            access_token: access_token.into(),
        })
        .build()
        .unwrap();

    commands.spawn(ReqwestRequest((client, request)));
}

pub fn cleanup(commands: &mut Commands) {
    commands.spawn(StopHyperListener(5000));
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
                    .access_token = request.access_token.clone();
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
