use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, _app: &mut App) {
        /*app.add_state::<MenuState>()
        .add_system(menu_setup.in_schedule(OnEnter(GameState::Menu)))
        .add_systems((
            main_menu_setup.in_schedule(OnEnter(MenuState::Main)),
            despawn_screen::<OnMainMenuScreen>.in_schedule(OnExit(MenuState::Main)),
        ))
        .add_systems((menu_action, button_system).in_set(OnUpdate(GameState::Menu)));*/
    }
}
