use std::time::Duration;

use bevy::prelude::*;

use crate::global::AimType;

pub mod rifle;
pub mod shotgun;
pub mod spawner;

const DEFAULT_WEAPON_COOLDOWN: u64 = 10;

const DEFAULT_RIFLE_SPREAD: u32 = 15;
const DEFAULT_RIFLE_DAMAGE: u32 = 1;
const DEFAULT_RIFLE_COOLDOWN: u32 = 1000;
const DEFAULT_RIFLE_BULLET_COUNT: i32 = 1;
const DEFAULT_RIFLE_AIM_TYPE: AimType = AimType::Closest;

const DEFAULT_SHOTGUN_BULLET_COUNT: i32 = 8;
const DEFAULT_SHOTGUN_SPREAD: u32 = 15;
const DEFAULT_SHOTGUN_DAMAGE: u32 = 1;
const DEFAULT_SHOTGUN_COOLDOWN: u32 = 1;
const DEFAULT_SHOTGUN_AIM_TYPE: AimType = AimType::Closest;

#[derive(Component)]
pub struct Weapon {
    pub aim_type: AimType,
    pub damage: u32,
    pub cooldown: u32,
    pub spread: u32,
    pub bullet_count: i32,
}

#[derive(Component)]
pub struct WeaponCoolDown {
    pub timer: Timer,
}

impl Default for WeaponCoolDown {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_millis(DEFAULT_WEAPON_COOLDOWN),
                TimerMode::Repeating,
            ),
        }
    }
}
