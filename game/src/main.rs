#![deny(warnings)]

// https://github.com/bevyengine/bevy/blob/v0.10.0/examples/games/game_menu.rs

mod auth;
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
    //app.add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default());

    #[cfg(feature = "client")]
    {
        println!("starting client");

        // core bevy plugins
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "ssjJune2023".to_owned(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    //watch_for_changes: true,
                    ..Default::default()
                })
                .set(bevy::log::LogPlugin {
                    level: LOG_LEVEL,
                    ..Default::default()
                }),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        ));

        // egui
        app.add_plugins(bevy_egui::EguiPlugin);

        // inspectors
        app.add_plugins((
            bevy_inspector_egui::quick::WorldInspectorPlugin::default().run_if(
                bevy::input::common_conditions::input_toggle_active(true, KeyCode::Grave),
            ),
            bevy_inspector_egui::quick::StateInspectorPlugin::<GameState>::default().run_if(
                bevy::input::common_conditions::input_toggle_active(true, KeyCode::Grave),
            ),
            bevy_inspector_egui::quick::StateInspectorPlugin::<
                plugins::client::main_menu::MainMenuState,
            >::default()
            .run_if(bevy::input::common_conditions::input_toggle_active(
                true,
                KeyCode::Grave,
            )),
        ));

        // client plugins
        app.add_plugins((
            plugins::client::splash::SplashPlugin,
            plugins::client::main_menu::MainMenuPlugin,
        ));

        app.add_systems(Startup, systems::client::startup);
    }

    #[cfg(feature = "server")]
    {
        println!("starting server");

        // core bevy plugins
        app.add_plugins((
            MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_loop(
                bevy::utils::Duration::from_secs_f64(1.0 / 60.0),
            )),
            // TODO: do we need the asset plugin?
            bevy::log::LogPlugin {
                level: LOG_LEVEL,
                ..Default::default()
            },
        ));

        // server plugins
        app.add_plugins((
            plugins::server::aws::AwsTaskPlugin,
            plugins::server::init::InitServerPlugin,
            plugins::server::looking_for_work::LookingForWorkPlugin,
            plugins::server::working::WorkingPlugin,
        ));
    }

    // shared plugins
    app.add_plugins((
        bevy_tokio_tasks::TokioTasksPlugin::default(),
        plugins::reqwest::ReqwestPlugin,
        plugins::hyper::HyperPlugin,
    ));

    // shared resources
    app.insert_resource(Random::default());

    app.run();
}
