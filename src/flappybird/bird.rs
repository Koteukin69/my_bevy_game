use crate::flappybird::physics::{ApplyGravity, Position, Velocity};
use bevy::prelude::*;

const JUMP_VELOCITY: f32 = 10.0;

#[derive(Component)]
pub struct Bird;

pub fn spawn_bird(mut commands: Commands) {
    commands.spawn((
        Bird,
        Position(Vec2::new(0.0, 0.0)),
        Velocity(Vec2::new(0.0, 10.0)),
        ApplyGravity,
    ));
}

pub fn handle_jump(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Velocity, With<Bird>>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut vel in &mut query {
            vel.0.y = JUMP_VELOCITY;
        }
    }
}
