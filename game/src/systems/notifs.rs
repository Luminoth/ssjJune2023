use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use futures_lite::future;

use crate::components::notifs::*;

pub fn subscribe_notifs(
    mut _commands: Commands,
    requests: Query<(Entity, &SubscribeNotifs), Added<SubscribeNotifs>>,
    _runtime: Res<TokioTasksRuntime>,
) {
    for (_entity, _request) in requests.iter() {
        /*let request = request.0.clone();
        let uri = request.uri().clone();
        let task = runtime.spawn_background_task(|mut ctx| async move {
            let (stream, _) = tokio_tungstenite::connect_async(request).await?;

            ctx.run_on_main_thread(move |ctx| {
                ctx.world.spawn(ListenNotifs((uri, stream)));
            })
            .await;

            Ok(())
        });

        commands
            .entity(entity)
            .insert(SubscribeNotifsTask((uri, task)))
            .remove::<SubscribeNotifs>();*/
    }
}

pub fn poll_subscribe_notifs(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut SubscribeNotifsTask)>,
) {
    for (entity, mut task) in tasks.iter_mut() {
        if let Some(response) = future::block_on(future::poll_once(&mut task.0 .1)) {
            // TODO: error handling
            let response = response.unwrap();

            // TODO: error handling
            response.unwrap();

            debug!("subscribed to notifs from {}", task.0 .0);

            commands.entity(entity).despawn();
        }
    }
}

pub fn listen_notifs(
    mut commands: Commands,
    requests: Query<(Entity, &ListenNotifs), Added<ListenNotifs>>,
    runtime: Res<TokioTasksRuntime>,
) {
    for (entity, request) in requests.iter() {
        //let stream = request.0 .1.clone();
        let task = runtime.spawn_background_task(|_ctx| async move {
            // TODO: forever await the stream

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

            debug!("unsubscribed from notifs from {}", task.0 .0);

            commands.entity(entity).despawn();
        }
    }
}
