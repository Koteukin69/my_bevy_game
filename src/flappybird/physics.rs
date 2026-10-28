use bevy::prelude::*;

const GRAVITY: f32 = -40.0;

#[derive(Component)]
pub struct Position(pub Vec2);

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct ApplyGravity;

#[derive(Component)]
pub struct ClampY {
    pub min: f32,
    pub max: f32,
}

pub fn clamp_position_y(mut query: Query<(&mut Position, &ClampY)>) {
    for (mut pos, clamp) in &mut query {
        pos.0.y = pos.0.y.clamp(clamp.min, clamp.max);
    }
}

pub fn apply_physics(time: Res<Time>, mut query: Query<(&mut Position, &Velocity)>) {
    for (mut pos, vel) in &mut query {
        pos.0 += vel.0 * time.delta_secs();
    }
}

pub fn apply_gravity(time: Res<Time>, mut query: Query<&mut Velocity, With<ApplyGravity>>) {
    for mut vel in &mut query {
        vel.0.y += time.delta_secs() * GRAVITY;
    }
}

pub fn move_rigidbody(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &Position,
        Option<&Velocity>,
        Option<&ClampY>,
    )>,
) {
    let dt = fixed_time.overstep().as_secs_f32();

    for (mut transform, pos, vel, clamp) in &mut query {
        let mut p = pos.0;
        if let Some(vel) = vel {
            p += vel.0 * dt;
        }
        if let Some(c) = clamp {
            p.y = p.y.clamp(c.min, c.max);
        }
        transform.translation = p.extend(transform.translation.z);
    }
}
