use bevy::prelude::*;

use crate::components::client::splash::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, client::splash::*};

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), enter)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            .add_systems(
                OnExit(GameState::Splash),
                (exit, cleanup_state::<OnSplashScreen>),
            );
    }
}
