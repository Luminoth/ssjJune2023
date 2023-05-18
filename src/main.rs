use bevy::prelude::*;

mod components;
mod plugins;
mod resources;
mod systems;

use plugins::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .run();
}
