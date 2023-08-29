use bevy::prelude::*;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

use crate::components::notifs::*;

pub fn subscribe(commands: &mut Commands, access_token: impl AsRef<str>) {
    info!("subscribing to notifications");

    let mut notifs_request = "ws://localhost:3000/notifs".into_client_request().unwrap();
    let headers = notifs_request.headers_mut();
    headers.insert(
        http::header::AUTHORIZATION,
        format!("Bearer {}", access_token.as_ref()).parse().unwrap(),
    );
    commands.spawn(SubscribeNotifs(Some(notifs_request)));
}

// TODO: unsubscribe
