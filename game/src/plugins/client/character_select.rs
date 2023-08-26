use bevy::prelude::*;

use crate::components::client::character_select::*;
use crate::events::client::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, client::character_select::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum CharacterSelectState {
    #[default]
    Init,
    WaitForCharacters,
    WaitForCharacterSelect,
}

pub struct CharacterSelectPlugin;

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<CharacterSelectState>()
            .add_event::<CharactersUpdated>()
            .add_systems(OnEnter(GameState::CharacterSelect), enter)
            .add_systems(
                Update,
                (
                    wait_for_characters.run_if(in_state(CharacterSelectState::WaitForCharacters)),
                    wait_for_character_select
                        .run_if(in_state(CharacterSelectState::WaitForCharacterSelect)),
                ),
            )
            .add_systems(
                OnExit(GameState::CharacterSelect),
                (exit, cleanup_state::<OnCharacterSelect>),
            );
    }
}
