use bevy::prelude::*;
use bevy_tokio_tasks::*;
use futures_lite::future;

use crate::components::server::init::*;
use crate::resources::server::*;
use crate::states::GameState;

pub fn setup(mut commands: Commands, runtime: Res<TokioTasksRuntime>) {
    info!("entering InitServer state");

    let task = runtime.spawn_background_task(|_ctx| async move {
        info!("loading AWS config...");
        aws_config::load_from_env().await
    });

    commands.spawn((AwsConfigTask(task), OnInitServer));
}

pub fn teardown(mut commands: Commands, to_despawn: Query<Entity, With<OnInitServer>>) {
    info!("exiting InitServer state");

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn wait_for_aws_config_task(
    mut commands: Commands,
    mut config_tasks: Query<(Entity, &mut AwsConfigTask)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (entity, mut task) = config_tasks.single_mut();
    if let Some(config) = future::block_on(future::poll_once(&mut task.0)) {
        // TODO: error handling
        let config = config.unwrap();

        debug!("AWS config: {:?}", config);

        commands.entity(entity).despawn_recursive();
        commands.insert_resource(AwsConfig(config));

        game_state.set(GameState::LookingForWork);
    }
}
