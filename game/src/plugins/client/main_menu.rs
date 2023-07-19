use bevy::prelude::*;

use crate::components::client::main_menu::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, client::main_menu::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum MainMenuState {
    #[default]
    Init,
    WaitForAuth,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_systems(OnEnter(GameState::MainMenu), enter)
            .add_systems(
                Update,
                (wait_for_auth.run_if(in_state(MainMenuState::WaitForAuth)),),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                (exit, cleanup_state::<OnMainMenu>),
            );
    }
}
