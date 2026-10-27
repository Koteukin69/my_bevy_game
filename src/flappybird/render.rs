use crate::flappybird::physics::Position;
use bevy::camera::ScalingMode;
use bevy::prelude::*;

const VIEWPORT_HEIGHT_UNITS: f32 = 10.0;

pub(super) fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: VIEWPORT_HEIGHT_UNITS,
            },
            ..OrthographicProjection::default_2d()
        }),
    ));
}

pub fn move_rigidbody(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, pos) in &mut query {
        transform.translation = (pos.0).extend(transform.translation.z);
    }
}
