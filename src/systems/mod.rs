use bevy::prelude::*;

use crate::components::*;
use crate::resources::*;

pub fn add_people(mut commands: Commands) {
    commands.spawn((
        Person,
        crate::components::Name("Elaina Proctor".to_string()),
    ));
    commands.spawn((Person, crate::components::Name("Renzo Hume".to_string())));
    commands.spawn((Person, crate::components::Name("Zayna Nieves".to_string())));
}

pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&crate::components::Name, With<Person>>,
) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}
