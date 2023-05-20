// https://github.com/bevyengine/bevy/blob/main/examples/games/game_menu.rs

use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod states;
mod systems;

use plugins::*;
use states::*;

/*fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}*/

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello World!".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        //.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .add_plugin(HelloPlugin)
        .add_state::<GameState>()
        /*.insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)*/
        .run();
}
