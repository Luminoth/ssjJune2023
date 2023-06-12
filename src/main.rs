// https://github.com/bevyengine/bevy/blob/v0.10.0/examples/games/game_menu.rs

mod components;
mod plugins;
mod resources;
mod states;
mod systems;

use bevy::prelude::*;

use states::*;

/*fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}*/

fn main() {
    #[cfg(all(feature = "client", feature = "server"))]
    compile_error!("feature \"client\" and feature \"server\" cannot be enabled at the same time");

    #[cfg(feature = "client")]
    println!("starting client");

    #[cfg(feature = "server")]
    println!("starting server");

    let mut app = App::new();
    app.add_state::<GameState>();

    #[cfg(feature = "client")]
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "ssjJune2023".into(),
            ..default()
        }),
        ..default()
    }))
    //.insert_resource(ClearColor(Color::BLACK))
    .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
    .add_plugin(plugins::splash::SplashPlugin);

    #[cfg(feature = "server")]
    app.insert_resource(bevy::app::ScheduleRunnerSettings::run_loop(
        bevy::utils::Duration::from_secs_f64(1.0 / 60.0),
    ))
    .add_plugins(MinimalPlugins);

    app
        //.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        //.add_startup_system(setup)
        .run();
}
