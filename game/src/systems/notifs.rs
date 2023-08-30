use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use futures_lite::future;
use futures_util::stream::StreamExt;

use crate::components::notifs::*;
use crate::events::notifs::*;

pub fn subscribe_notifs(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut SubscribeNotifs), Added<SubscribeNotifs>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, mut request) in requests.iter_mut() {
        if request.0.is_none() {
            continue;
        }

        let request = request.0.take().unwrap();
        let uri = request.uri().clone();
        let task = runtime.spawn_background_task(|mut ctx| async move {
            let uri = request.uri().clone();

            // TODO: error handle this (if it fails, send a NotifsSubscribeResult event)
            let (stream, _) = tokio_tungstenite::connect_async(request).await?;

            ctx.run_on_main_thread(move |ctx| {
                ctx.world.spawn(ListenNotifs((uri, Some(stream))));
            })
            .await;

            Ok(())
        });

        commands
            .entity(entity)
            .insert(SubscribeNotifsTask((uri, task)))
            .remove::<SubscribeNotifs>();
    }
}

pub fn poll_subscribe_notifs(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SubscribeNotifsTask)>,
    mut notifs_subscribed_events: EventWriter<NotifsSubscribeResult>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(response) = future::block_on(future::poll_once(&mut task.0 .1)) {
            // TODO: error handling
            let response = response.unwrap();

            // TODO: error handling
            response.unwrap();

            notifs_subscribed_events.send(NotifsSubscribeResult((task.0 .0.clone(), true)));

            debug!("subscribed to notifications from {}", task.0 .0);

            commands.entity(entity).despawn();
        }
    }
}

pub fn listen_notifs(
    mut commands: Commands,
    mut requests: Query<(Entity, &mut ListenNotifs), Added<ListenNotifs>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, mut request) in requests.iter_mut() {
        let uri = request.0 .0.clone();
        let stream = request.0 .1.take().unwrap();
        let task = runtime.spawn_background_task(|mut ctx| async move {
            let (_, mut read) = stream.split();
            while let Some(Ok(msg)) = read.next().await {
                let uri = uri.clone();
                info!("got notification from {}: {}", uri, msg);
                ctx.run_on_main_thread(move |ctx| {
                    ctx.world.send_event(Notification((uri, msg)));
                })
                .await;
            }

            warn!("{} notifications connection closed", uri);
            ctx.run_on_main_thread(move |ctx| {
                ctx.world.send_event(NotifsDisconnected(uri));
            })
            .await;

            Ok(())
        });

        commands
            .entity(entity)
            .insert(ListenNotifsTask((request.0 .0.clone(), task)))
            .remove::<SubscribeNotifs>();
    }
}

pub fn poll_listen_notifs(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut ListenNotifsTask)>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(response) = future::block_on(future::poll_once(&mut task.0 .1)) {
            // TODO: error handling
            let response = response.unwrap();

            // TODO: error handling
            response.unwrap();

            debug!(
                "finished listening for notifications from from {}",
                task.0 .0
            );

            commands.entity(entity).despawn();
        }
    }
}
