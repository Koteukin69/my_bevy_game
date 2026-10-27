use crate::flappybird::physics::{ApplyGravity, YPos, YVelocity};
use bevy::prelude::*;

const JUMP_VELOCITY: f32 = 10.0;

#[derive(Component)]
pub struct Bird;

pub fn spawn_bird(mut commands: Commands) {
    commands.spawn((Bird, YPos(0.0), YVelocity(10.0), ApplyGravity));
}

pub fn handle_jump(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut YVelocity, With<Bird>>) {
    if keys.just_pressed(KeyCode::Space) {
        for mut vel in &mut query {
            vel.0 = JUMP_VELOCITY;
        }
    }
}
