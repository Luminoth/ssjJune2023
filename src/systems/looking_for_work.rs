#![cfg(feature = "server")]

use bevy::prelude::*;

use crate::components::looking_for_work::*;
use crate::states::GameState;

pub fn setup(mut _commands: Commands) {
    info!("Entering Looking For Work state");
}

pub fn teardown(to_despawn: Query<Entity, With<OnLookingForWork>>, mut commands: Commands) {
    info!("Exiting Looking For Work state");

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn look_for_work(mut _game_state: ResMut<NextState<GameState>>) {}
