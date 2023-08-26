use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::components::client::splash::*;
use crate::resources::client::splash::*;
use crate::states::GameState;

pub fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("entering Splash state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnSplashScreen));

    let image = asset_server.load("images/splash.png");

    // TODO: fade-in / fade-out
    // TODO: multiple splash screens (PIGSquad, Bevy)

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnSplashScreen,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(200.0),
                    ..default()
                },
                image: UiImage::new(image),
                ..default()
            });
        });

    commands.insert_resource(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

pub fn exit() {
    info!("exiting Splash state");
}

pub fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    mut contexts: EguiContexts,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::MainMenu);
    }

    // TODO: this is just for debugging
    // while there is no actual splash
    egui::Window::new("Splash").show(contexts.ctx_mut(), |ui| {
        ui.label("Waiting for splash ...");
    });
}
