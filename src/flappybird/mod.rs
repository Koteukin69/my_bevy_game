use bevy::prelude::*;

mod bird;
mod physics;
mod render;

pub struct FlappyBirdPlugin;

impl Plugin for FlappyBirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (bird::spawn_bird, render::spawn_camera));
        app.add_systems(
            FixedUpdate,
            (
                physics::apply_gravity,
                physics::apply_physics,
                physics::clamp_position_y,
                render::move_rigidbody,
            )
                .chain(),
        );
        app.add_systems(Update, (bird::handle_jump, physics::debug_pos));
    }
}
