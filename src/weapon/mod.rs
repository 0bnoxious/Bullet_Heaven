use bevy::prelude::*;

use crate::global::AimType;

pub mod rifle;
pub mod shotgun;

#[derive(Component)]
pub struct Weapon {
    pub aim_type: AimType,
    pub damage: f64,
    pub fire_rate: f64,
    pub range: f64,
    pub spread: f64,
}
