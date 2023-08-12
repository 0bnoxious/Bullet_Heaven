use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{
    global::AimType,
    mob::infected::Infected,
    player::*,
    projectile::{spawner::ProjectileSpawner, Projectile},
    targeting::{define_spread, HasTarget},
};

pub const DEFAULT_SHOTGUN_BULLET_COUNT: u32 = 8;
pub const DEFAULT_SHOTGUN_SPREAD: u32 = 15;
pub const DEFAULT_SHOTGUN_DAMAGE: i32 = 1;
pub const DEFAULT_SHOTGUN_COOLDOWN: u32 = 1;
pub const DEFAULT_SHOTGUN_AIM_TYPE: AimType = AimType::Closest;

#[derive(Component)]
pub struct Shotgun {
    pub bullet_count: u32,
    pub spread: u32,
    pub damage: i32,
    pub cooldown: u32,
}

impl Default for Shotgun {
    fn default() -> Self {
        Self {
            bullet_count: DEFAULT_SHOTGUN_BULLET_COUNT,
            spread: DEFAULT_SHOTGUN_SPREAD,
            damage: DEFAULT_SHOTGUN_DAMAGE,
            cooldown: DEFAULT_SHOTGUN_COOLDOWN,
        }
    }
}

#[derive(Component)]
pub struct ShotgunCoolDown {
    pub timer: Timer,
}

impl Default for ShotgunCoolDown {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_millis(DEFAULT_SHOTGUN_COOLDOWN as u64),
                TimerMode::Repeating,
            ),
        }
    }
}

#[derive(Bundle)]
pub struct ShotgunBundle {
    pub shotgun: Shotgun,
    pub cooldown: ShotgunCoolDown,
    pub aim_type: AimType,
}

impl Default for ShotgunBundle {
    fn default() -> Self {
        Self {
            shotgun: default(),
            cooldown: default(),
            aim_type: DEFAULT_SHOTGUN_AIM_TYPE,
        }
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn fire_shotgun(
    mut shotgun_query: Query<(&Shotgun, &mut ShotgunCoolDown, &AimType)>,
    mut target_query: Query<(&HasTarget, &Position), (With<Player>, Without<Projectile>)>,
    infected_position_query: Query<&Infected>,
    mut projectile_spawner: ProjectileSpawner,
    time: Res<Time>,
) {
    if !shotgun_query.is_empty() {
        for (shotgun, mut shotgun_cooldown, shotgun_aimtype) in &mut shotgun_query {
            shotgun_cooldown.timer.tick(time.delta());
            if !infected_position_query.is_empty() && shotgun_cooldown.timer.finished() {
                for (player_has_target, player_position) in &mut target_query {
                    for _ in 0..shotgun.bullet_count {
                        let spread = define_spread(
                            player_position.0,
                            player_has_target.target_position,
                            shotgun.spread,
                        );
                        projectile_spawner.spawn_shotgun_projectile(
                            player_position.0,
                            spread,
                            shotgun_aimtype.clone(),
                        )
                    }
                }
            }
        }
    }
}
