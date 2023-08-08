use bevy::prelude::*;

use crate::global::AimType;

/*pub const DEFAULT_WEAPON_DAMAGE: f64 = 1.;
pub const DEFAULT_WEAPON_FIRE_RATE: f64 = 1000.;
pub const DEFAULT_WEAPON_RANGE: f64 = 100.;
pub const DEFAULT_WEAPON_SPREAD: f64 = 1.;
pub const DEFAULT_WEAPON_AIM_TYPE: AimType = AimType::Closest;*/

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

/*pub fn default_weapon() -> Weapon {
    Weapon {
        aim_type: DEFAULT_WEAPON_AIM_TYPE,
        range: DEFAULT_WEAPON_DAMAGE,
        damage: DEFAULT_WEAPON_FIRE_RATE,
        fire_rate: DEFAULT_WEAPON_RANGE,
        spread: DEFAULT_WEAPON_SPREAD,
    }
}*/
