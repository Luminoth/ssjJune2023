use bevy::prelude::*;

pub mod client;
pub mod server;

pub fn cleanup_state<T>(mut commands: Commands, query: Query<Entity, With<T>>)
where
    T: Component,
{
    for e in &query {
        commands.entity(e).despawn_recursive();
    }

    commands.remove_resource::<ClearColor>();
}
