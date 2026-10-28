use bevy::prelude::*;

mod bird;
mod physics;
mod pipes;
mod render;

pub struct FlappyBirdPlugin;

impl Plugin for FlappyBirdPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(pipes::SpawnerTimer(Timer::from_seconds(
            2.0,
            TimerMode::Repeating,
        )));
        app.add_systems(
            Startup,
            (
                bird::spawn_bird,
                render::spawn_camera,
                render::spawn_letterbox,
            ),
        );
        app.add_systems(
            FixedUpdate,
            (
                physics::apply_gravity,
                physics::apply_physics,
                physics::clamp_position_y,
            )
                .chain(),
        );
        app.add_systems(
            Update,
            (
                (bird::handle_jump, pipes::pipes_spawner),
                render::move_rigidbody,
            )
                .chain(),
        );
    }
}
