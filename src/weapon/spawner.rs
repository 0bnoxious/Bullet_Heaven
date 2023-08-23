use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_xpbd_2d::prelude::Position;

use super::{rifle::Rifle, shotgun::Shotgun, *};

#[derive(SystemParam)]
pub struct WeaponSpawner<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'w, 's> WeaponSpawner<'w, 's> {
    pub fn spawn_rifle(&mut self) {
        self.commands.spawn((
            Weapon {
                aim_type: DEFAULT_RIFLE_AIM_TYPE,
                damage: DEFAULT_RIFLE_DAMAGE,
                cooldown: DEFAULT_RIFLE_COOLDOWN,
                spread: DEFAULT_RIFLE_SPREAD,
                bullet_count: DEFAULT_RIFLE_BULLET_COUNT,
            },
            Rifle,
            WeaponCoolDown {
                timer: Timer::new(
                    Duration::from_millis(DEFAULT_RIFLE_COOLDOWN as u64),
                    TimerMode::Repeating,
                ),
            },
            Position(Vec2::ZERO),
            Name::new("Rifle"),
        ));
    }

    pub fn spawn_shotgun(&mut self) {
        self.commands.spawn((
            Weapon {
                aim_type: DEFAULT_SHOTGUN_AIM_TYPE,
                damage: DEFAULT_SHOTGUN_DAMAGE,
                cooldown: DEFAULT_SHOTGUN_COOLDOWN,
                spread: DEFAULT_SHOTGUN_SPREAD,
                bullet_count: DEFAULT_SHOTGUN_BULLET_COUNT,
            },
            WeaponCoolDown {
                timer: Timer::new(
                    Duration::from_millis(DEFAULT_SHOTGUN_COOLDOWN as u64),
                    TimerMode::Repeating,
                ),
            },
            Shotgun,
            Position(Vec2::ZERO),
            Name::new("Shotgun"),
        ));
    }
}
