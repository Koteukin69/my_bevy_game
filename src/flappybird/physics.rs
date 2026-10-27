use bevy::prelude::*;

const GRAVITY: f32 = -20.0;

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

pub fn debug_pos(query: Query<&Position>) {
    for pos in &query {
        println!("X: {:.2}, Y: {:.2}", pos.0.x, pos.0.y);
    }
}
