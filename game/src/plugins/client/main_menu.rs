use bevy::prelude::*;

use crate::components::client::main_menu::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, client::main_menu::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum MainMenuState {
    #[default]
    Init,
    WaitForLogin,
    WaitForOAuth,
    WaitForAuth,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_system(enter.in_schedule(OnEnter(GameState::MainMenu)))
            .add_systems((wait_for_login.in_set(OnUpdate(MainMenuState::WaitForLogin)),))
            .add_systems((wait_for_oauth.in_set(OnUpdate(MainMenuState::WaitForOAuth)),))
            .add_systems((wait_for_auth.in_set(OnUpdate(MainMenuState::WaitForAuth)),))
            .add_system(exit.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(cleanup_state::<OnMainMenu>.in_schedule(OnExit(GameState::MainMenu)));
    }
}
