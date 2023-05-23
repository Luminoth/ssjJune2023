use bevy::prelude::*;

use crate::states::GameState;
use crate::systems::splash::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(GameState::Splash)))
            .add_system(countdown.in_set(OnUpdate(GameState::Splash)))
            .add_system(teardown.in_schedule(OnExit(GameState::Splash)));
    }
}
