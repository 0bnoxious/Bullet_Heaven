use bevy::prelude::*;

pub mod projectile_spawner;

pub const PROJECTILE_SPEED: f32 = 200.;
pub const PROJECTILE_SIZE: f32 = 8.;
pub const PROJECTILE_LIFE_SPAN: u64 = 3;

#[derive(Component)]
pub struct ProjectileTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Projectile;
