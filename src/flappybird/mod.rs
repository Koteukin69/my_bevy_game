use bevy::prelude::*;

mod bird;
mod physics;

pub struct FlappyBirdPlugin;

impl Plugin for FlappyBirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, bird::spawn_bird);
        app.add_systems(
            FixedUpdate,
            (physics::apply_gravity, physics::apply_physics).chain(),
        );
        app.add_systems(Update, (bird::handle_jump, physics::debug_pos));
    }
}
