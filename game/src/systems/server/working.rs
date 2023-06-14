use bevy::prelude::*;

use crate::components::server::working::*;
use crate::resources::server::*;
use crate::states::GameState;

pub fn setup(
    mut commands: Commands,
    _message: Res<WorkMessage>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    info!("entering Working state");

    // TODO: handle the work and start doing stuff
    commands.remove_resource::<WorkMessage>();
    game_state.set(GameState::LookingForWork);
}

pub fn teardown(mut commands: Commands, to_despawn: Query<Entity, With<OnWorking>>) {
    info!("exiting Working state");

    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
