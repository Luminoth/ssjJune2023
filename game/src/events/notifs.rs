use bevy::prelude::*;
use http::uri::Uri;

#[derive(Event)]
pub struct NotifsSubscribeResult(pub (Uri, bool));

#[derive(Event)]
pub struct NotifsDisconnected(pub Uri);

#[derive(Event)]
pub struct Notification(pub (Uri, tokio_tungstenite::tungstenite::protocol::Message));
