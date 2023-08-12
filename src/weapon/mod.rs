use bevy::prelude::*;

use crate::global::AimType;

pub mod rifle;
pub mod shotgun;

#[derive(Component)]
pub struct Weapon {
    pub aim_type: AimType,
    pub damage: u32,
    pub fire_rate: u32,
    pub spread: u32,
}
