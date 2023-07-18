use bevy::prelude::*;

use crate::components::server::{aws::*, init::*};
use crate::resources::server::*;
use crate::states::GameState;

pub fn enter(mut commands: Commands) {
    info!("entering InitServer state");

    commands.spawn((LoadAwsConfigRequest, OnInitServer));
}

pub fn exit() {
    info!("exiting InitServer state");
}

pub fn wait_for_aws_config_task(
    mut commands: Commands,
    mut results: Query<(Entity, &mut LoadAwsConfigResult)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok((entity, mut result)) = results.get_single_mut() {
        // TODO: error handling
        let config = result.get_result_mut().take().unwrap();

        commands.insert_resource(AwsConfig(config));
        commands.entity(entity).despawn_recursive();

        game_state.set(GameState::LookingForWork);
    }
}
