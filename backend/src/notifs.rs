use axum::extract::ws::WebSocket;
use futures_util::stream::StreamExt;
use tracing::info;

pub async fn handle_notifs(socket: WebSocket, user_id: impl AsRef<str>) {
    let user_id = user_id.as_ref();
    info!("{} subscribed to notifications ...", user_id);

    let (mut _sender, mut receiver) = socket.split();

    let mut send_task = tokio::spawn(async move {
        loop {
            // TODO:

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }
    });

    // receive task just lets us know when the connection is closed
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = receiver.next().await {
            // ignore whatver we received
        }
    });

    tokio::select! {
        _ = (&mut send_task) => {
             info!("closed notifications connection from {}", user_id);
            recv_task.abort();
        },
        _ = (&mut recv_task) => {
            info!("{} closed notifications connection", user_id);
            send_task.abort();
        }
    }
}
