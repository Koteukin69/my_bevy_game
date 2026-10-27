use crate::flappybird::physics::{Position, Velocity};
use bevy::prelude::*;

const PIPE_WIDTH: f32 = 2.0;
const PIPE_HEIGHT: f32 = 10.0;
const GAP: f32 = 4.0;
const MOVE_SPEED: f32 = 5.0;
const OFFSET: f32 = (PIPE_HEIGHT + GAP) / 2.0;

#[derive(Component)]
pub struct Pipes;

pub fn spawn_pipes(mut commands: Commands) {
    commands.spawn((
        Pipes,
        Position(Vec2::new(20.0, 0.0)),
        Velocity(Vec2::new(-MOVE_SPEED, 0.0)),
        children![
            (
                Sprite::from_color(
                    Color::srgb(0.0, 1.0, 0.0),
                    Vec2::new(PIPE_WIDTH, PIPE_HEIGHT)
                ),
                Transform::from_xyz(0.0, OFFSET, 0.0)
            ),
            (
                Sprite::from_color(
                    Color::srgb(0.0, 1.0, 0.0),
                    Vec2::new(PIPE_WIDTH, PIPE_HEIGHT)
                ),
                Transform::from_xyz(0.0, -OFFSET, 0.0)
            ),
        ],
        Transform::from_xyz(0.0, 0.0, 0.0),
        Visibility::default(),
    ));
}
