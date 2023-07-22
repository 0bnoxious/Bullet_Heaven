use bevy::prelude::*;

use crate::global::*;

pub mod player_spawner;

pub const PLAYER_SIZE: f32 = 10.;
pub const ATTACK_SPEED: u64 = 100;
pub const PLAYER_SPEED: f32 = 1.;

#[derive(Component)]
pub struct Player {
    pub aim_type: AimType,
}

#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}
