use bevy::prelude::*;
use bevy_tokio_tasks::*;

use crate::components::server::working::*;

pub fn setup(mut _commands: Commands, _runtime: ResMut<TokioTasksRuntime>) {
    info!("entering Working state");
}

pub fn teardown(mut commands: Commands, to_despawn: Query<Entity, With<OnWorking>>) {
    info!("exiting Working state");

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
