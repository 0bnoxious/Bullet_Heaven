use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_xpbd_2d::prelude::Position;

use crate::{mob::infected::Infected, player::Player};

pub const DEFAULT_WEAPON_DAMAGE: f64 = 1.;
pub const DEFAULT_WEAPON_FIRE_RATE: f64 = 1000.;
pub const DEFAULT_WEAPON_RANGE: f64 = 100.;
pub const DEFAULT_WEAPON_SPREAD: f64 = 1.;

pub mod shotgun;

#[derive(Component)]
pub struct Weapon {
    pub damage: f64,
    pub fire_rate: f64,
    pub range: f64,
    pub spread: f64,
}

pub fn default_weapon() -> Weapon {
    Weapon {
        range: DEFAULT_WEAPON_DAMAGE,
        damage: DEFAULT_WEAPON_FIRE_RATE,
        fire_rate: DEFAULT_WEAPON_RANGE,
        spread: DEFAULT_WEAPON_SPREAD,
    }
}
