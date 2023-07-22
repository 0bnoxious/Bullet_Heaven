use bevy::prelude::*;

pub mod projectile_spawner;

pub const PROJECTILE_SIZE: f32 = 8.;
pub const PROJECTILE_SPEED: f32 = 200.;
pub const PROJECTILE_DAMAGE: i32 = 1;
pub const PROJECTILE_LIFE_SPAN: u64 = 20;

#[derive(Component)]
pub struct ProjectileTimer {
    pub timer: Timer,
}

#[derive(Component, Debug)]
pub struct Projectile;
