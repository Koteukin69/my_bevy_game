use bevy::prelude::*;

mod components;
mod systems;

use components::GreetTimer;
use systems::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        println!("HelloPlugin build");
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, (hello_world, add_people));
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}
