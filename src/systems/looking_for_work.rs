#![cfg(feature = "server")]

use bevy::prelude::*;

use crate::components::looking_for_work::*;
use crate::states::GameState;

pub fn setup(mut _commands: Commands) {
    info!("entering LookingForWork state");
}

pub fn teardown(mut commands: Commands, to_despawn: Query<Entity, With<OnLookingForWork>>) {
    info!("exiting LookingForWork state");

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn look_for_work(mut _game_state: ResMut<NextState<GameState>>) {}
