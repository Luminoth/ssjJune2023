// https://github.com/bevyengine/bevy/blob/main/examples/games/game_menu.rs

mod components;
mod plugins;
mod resources;
mod states;
mod systems;

use bevy::prelude::*;

use plugins::splash::*;
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
        .add_plugin(SplashPlugin)
        .add_state::<GameState>()
        /*.insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(setup)*/
        .run();
}
