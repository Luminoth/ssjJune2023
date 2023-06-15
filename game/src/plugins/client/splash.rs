use bevy::prelude::*;

use crate::components::client::splash::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, client::splash::*};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(enter.in_schedule(OnEnter(GameState::Splash)))
            .add_system(countdown.in_set(OnUpdate(GameState::Splash)))
            .add_system(exit.in_schedule(OnExit(GameState::Splash)))
            .add_system(cleanup_state::<OnSplashScreen>.in_schedule(OnExit(GameState::Splash)));
    }
}
