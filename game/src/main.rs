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

const LOG_LEVEL: bevy::log::Level = bevy::log::Level::INFO;

fn main() {
    #[cfg(all(feature = "client", feature = "server"))]
    compile_error!("feature \"client\" and feature \"server\" cannot be enabled at the same time");

    let mut app = App::new();

    // main game state has to come before plugins
    app.add_state::<GameState>();

    // core bevy plugins
    //app.add_plugin(bevy::diagnostic::LogDiagnosticsPlugin::default());

    #[cfg(feature = "client")]
    {
        println!("starting client");

        // core bevy plugins
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ssjJune2023".to_owned(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                })
                .set(bevy::log::LogPlugin {
                    level: LOG_LEVEL,
                    ..Default::default()
                }),
        )
        .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin);

        // egui
        app.add_plugin(bevy_egui::EguiPlugin);

        // inspector
        app.add_plugin(
            bevy_inspector_egui::quick::WorldInspectorPlugin::default().run_if(
                bevy::input::common_conditions::input_toggle_active(true, KeyCode::Grave),
            ),
        );

        // client plugins
        app.add_plugin(plugins::client::splash::SplashPlugin)
            .add_plugin(plugins::client::main_menu::MainMenuPlugin);
    }

    #[cfg(feature = "server")]
    {
        println!("starting server");

        // core bevy plugins
        app.insert_resource(bevy::app::ScheduleRunnerSettings::run_loop(
            bevy::utils::Duration::from_secs_f64(1.0 / 60.0),
        ))
        .add_plugins(MinimalPlugins)
        // TODO: do we need the asset plugin?
        .add_plugin(bevy::log::LogPlugin {
            level: LOG_LEVEL,
            ..Default::default()
        });

        // tokio runtime
        app.add_plugin(bevy_tokio_tasks::TokioTasksPlugin::default());

        // server plugins
        app.add_plugin(plugins::server::aws::AwsTaskPlugin)
            .add_plugin(plugins::server::init::InitServerPlugin)
            .add_plugin(plugins::server::looking_for_work::LookingForWorkPlugin)
            .add_plugin(plugins::server::working::WorkingPlugin);
    }

    // shared resources
    app.insert_resource(Random::default());

    app.run();
}
