use bevy::prelude::*;

use crate::components::client::main_menu::*;
use crate::events::client::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, client::main_menu::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum MainMenuState {
    #[default]
    Init,
    WaitForLogin,
    WaitForAuth,
    WaitForUser,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MainMenuState>()
            .add_event::<UserUpdated>()
            .add_systems(OnEnter(GameState::MainMenu), enter)
            .add_systems(
                Update,
                (
                    wait_for_login.run_if(in_state(MainMenuState::WaitForLogin)),
                    wait_for_auth.run_if(in_state(MainMenuState::WaitForAuth)),
                    wait_for_user.run_if(in_state(MainMenuState::WaitForUser)),
                ),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                (exit, cleanup_state::<OnMainMenu>),
            );
    }
}
