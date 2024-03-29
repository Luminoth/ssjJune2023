use bevy::prelude::*;

#[cfg(feature = "client")]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    CharacterSelect,
    Game,
}

#[cfg(feature = "server")]
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InitServer,
    LookingForWork,
    Working,
}
