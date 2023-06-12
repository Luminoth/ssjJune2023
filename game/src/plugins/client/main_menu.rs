use bevy::prelude::*;

use crate::states::GameState;
use crate::systems::client::main_menu::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MainMenuState {
    #[default]
    Main,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_system(setup.in_schedule(OnEnter(GameState::MainMenu)))
            // TODO: add main menu systems
            .add_system(teardown.in_schedule(OnExit(GameState::MainMenu)));
    }
}
