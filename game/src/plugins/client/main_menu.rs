use bevy::prelude::*;

use crate::components::client::main_menu::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, client::main_menu::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MainMenuState {
    #[default]
    Main,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_system(enter.in_schedule(OnEnter(GameState::MainMenu)))
            // TODO: add main menu systems
            .add_system(exit.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(cleanup_state::<OnMainMenu>.in_schedule(OnExit(GameState::MainMenu)));
    }
}
