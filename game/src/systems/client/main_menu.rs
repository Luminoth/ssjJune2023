use bevy::prelude::*;

use crate::components::client::main_menu::*;
use crate::plugins::client::main_menu::*;

pub fn enter(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    info!("entering MainMenu state");

    commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
    commands.spawn((Camera2dBundle::default(), OnMainMenu));

    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // login UI
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::width(Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            },
            OnMainMenu,
            LoginUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::SpaceBetween,
                        size: Size::new(Val::Auto, Val::Auto),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..Default::default()
                            },
                            LoginButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Login",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                ..Default::default()
                            });
                        });
                });
        });

    // OAuth input UI
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::width(Val::Percent(100.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            OnMainMenu,
            OAuthUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::SpaceBetween,
                        size: Size::new(Val::Auto, Val::Auto),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..Default::default()
                            },
                            OkButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Ok",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                ..Default::default()
                            });
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: Style {
                                    size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    ..Default::default()
                                },
                                background_color: Color::rgb(0.15, 0.15, 0.15).into(),
                                ..Default::default()
                            },
                            CancelButton,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle {
                                text: Text::from_section(
                                    "Cancel",
                                    TextStyle {
                                        font: font.clone(),
                                        font_size: 40.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                ),
                                ..Default::default()
                            });
                        });
                });
        });

    // waiting for auth UI
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    size: Size::width(Val::Percent(100.0)),
                    ..default()
                },
                visibility: Visibility::Hidden,
                ..default()
            },
            OnMainMenu,
            AuthUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::SpaceBetween,
                        size: Size::new(Val::Auto, Val::Auto),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|_parent| {
                    // TODO: waiting for auth text
                });
        });

    main_menu_state.set(MainMenuState::Main);
}

pub fn exit() {
    info!("exiting MainMenu state");
}

pub fn login_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<LoginButton>)>,
    mut login_ui: Query<&mut Visibility, (With<LoginUI>, Without<OAuthUI>, Without<AuthUI>)>,
    mut oauth_ui: Query<&mut Visibility, (With<OAuthUI>, Without<LoginUI>, Without<AuthUI>)>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if let Ok(interaction) = interaction_query.get_single_mut() {
        if *interaction == Interaction::Clicked {
            //webbrowser::open("https://itch.io/user/oauth?client_id=foobar&scope=profile:me&redirect_uri=urn:ietf:wg:oauth:2.0:oob").unwrap();

            if let Ok(mut login_ui) = login_ui.get_single_mut() {
                *login_ui = Visibility::Hidden;
            }

            if let Ok(mut oauth_ui) = oauth_ui.get_single_mut() {
                *oauth_ui = Visibility::Visible;
            }

            main_menu_state.set(MainMenuState::WaitForOAuth);
        }
    }
}

pub fn ok_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<OkButton>)>,
    mut oauth_ui: Query<&mut Visibility, (With<OAuthUI>, Without<LoginUI>, Without<AuthUI>)>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if let Ok(interaction) = interaction_query.get_single_mut() {
        if *interaction == Interaction::Clicked {
            // TODO: send auth request to backend

            if let Ok(mut oauth_ui) = oauth_ui.get_single_mut() {
                *oauth_ui = Visibility::Hidden;
            }

            main_menu_state.set(MainMenuState::WaitForAuth);
        }
    }
}

pub fn cancel_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<CancelButton>)>,
    mut login_ui: Query<&mut Visibility, (With<LoginUI>, Without<OAuthUI>, Without<AuthUI>)>,
    mut oauth_ui: Query<&mut Visibility, (With<OAuthUI>, Without<LoginUI>, Without<AuthUI>)>,
    mut main_menu_state: ResMut<NextState<MainMenuState>>,
) {
    if let Ok(interaction) = interaction_query.get_single_mut() {
        if *interaction == Interaction::Clicked {
            if let Ok(mut login_ui) = login_ui.get_single_mut() {
                *login_ui = Visibility::Visible;
            }

            if let Ok(mut oauth_ui) = oauth_ui.get_single_mut() {
                *oauth_ui = Visibility::Hidden;
            }

            main_menu_state.set(MainMenuState::Main);
        }
    }
}
