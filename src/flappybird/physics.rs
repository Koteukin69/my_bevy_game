use bevy::prelude::*;

const GRAVITY: f32 = -9.81;

#[derive(Component)]
pub struct YPos(pub f32);

#[derive(Component)]
pub struct YVelocity(pub f32);

#[derive(Component)]
pub struct ApplyGravity;

pub fn apply_physics(time: Res<Time>, mut query: Query<(&mut YPos, &YVelocity)>) {
    for (mut pos, vel) in &mut query {
        pos.0 += vel.0 * time.delta_secs();
    }
}

pub fn apply_gravity(time: Res<Time>, mut query: Query<&mut YVelocity, With<ApplyGravity>>) {
    for mut vel in &mut query {
        vel.0 += time.delta_secs() * GRAVITY;
    }
}

pub fn debug_pos(query: Query<&YPos>) {
    for pos in &query {
        println!("Y: {:.2}", pos.0);
    }
}
