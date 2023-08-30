use bevy::prelude::*;
use http::uri::Uri;
use tokio::net::TcpStream;
use tokio::task;
use tokio_tungstenite::{tungstenite::handshake::client::Request, MaybeTlsStream, WebSocketStream};

#[derive(Debug, Component)]
pub struct SubscribeNotifs(pub Option<Request>);

#[derive(Debug, Component)]
pub struct SubscribeNotifsTask(pub (Uri, task::JoinHandle<Result<(), anyhow::Error>>));

// TODO: unsubscribe

#[derive(Debug, Component)]
pub struct ListenNotifs(pub (Uri, Option<WebSocketStream<MaybeTlsStream<TcpStream>>>));

#[derive(Debug, Component)]
pub struct ListenNotifsTask(pub (Uri, task::JoinHandle<Result<(), anyhow::Error>>));
