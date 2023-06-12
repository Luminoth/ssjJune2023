use bevy::prelude::*;

#[cfg(feature = "client")]
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Game,
}

#[cfg(feature = "server")]
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    InitServer,
    LookingForWork,
    Working,
}
