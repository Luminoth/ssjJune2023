#![deny(warnings)]

// https://github.com/bevyengine/bevy/blob/v0.10.0/examples/games/game_menu.rs

mod components;
mod cooldown;
mod plugins;
mod resources;
mod states;
mod systems;

use bevy::prelude::*;

use resources::Random;
use states::*;

fn main() {
    #[cfg(all(feature = "client", feature = "server"))]
    compile_error!("feature \"client\" and feature \"server\" cannot be enabled at the same time");

    let mut app = App::new();
    app.add_state::<GameState>()
        //.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default())
        .insert_resource(Random::default());

    #[cfg(feature = "client")]
    {
        println!("starting client");
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "ssjJune2023".into(),
                ..default()
            }),
            ..default()
        }))
        //.insert_resource(ClearColor(Color::BLACK))
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugin(plugins::client::splash::SplashPlugin)
        .add_plugin(plugins::client::main_menu::MainMenuPlugin);
    }

    #[cfg(feature = "server")]
    {
        println!("starting server");
        app.insert_resource(bevy::app::ScheduleRunnerSettings::run_loop(
            bevy::utils::Duration::from_secs_f64(1.0 / 60.0),
        ))
        .add_plugins(MinimalPlugins)
        .add_plugin(bevy::log::LogPlugin::default())
        .add_plugin(bevy_tokio_tasks::TokioTasksPlugin::default())
        .add_plugin(plugins::server::init::InitServerPlugin)
        .add_plugin(plugins::server::looking_for_work::LookingForWorkPlugin)
        .add_plugin(plugins::server::working::WorkingPlugin);
    }

    app.run();
}
