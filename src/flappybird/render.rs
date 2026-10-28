use bevy::camera::ScalingMode;
use bevy::prelude::*;

const VIEWPORT_HEIGHT_UNITS: f32 = 20.0;
const FIELD_WIDTH: f32 = VIEWPORT_HEIGHT_UNITS * 3.0 / 4.0;

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

pub fn spawn_letterbox(mut commands: Commands) {
    for side in [-1.0, 1.0] {
        commands.spawn((
            Sprite::from_color(Color::BLACK, Vec2::splat(1000.0)),
            Transform::from_xyz(side * (FIELD_WIDTH / 2.0 + 500.0), 0.0, 100.0),
        ));
    }
}
