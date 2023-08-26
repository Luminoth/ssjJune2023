use bevy::prelude::*;

use crate::components::client::character_select::*;
use crate::states::GameState;
use crate::systems::{cleanup_state, client::character_select::*};

pub struct CharacterSelectPlugin;

impl Plugin for CharacterSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::CharacterSelect), enter)
            .add_systems(
                Update,
                temp_update.run_if(in_state(GameState::CharacterSelect)),
            )
            .add_systems(
                OnExit(GameState::CharacterSelect),
                (exit, cleanup_state::<OnCharacterSelect>),
            );
    }
}
