use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Position;

use crate::{
    global::AimType,
    mob::infected::Infected,
    player::*,
    projectile::{spawner::ProjectileSpawner, Projectile},
    targeting::{define_spread, HasTarget},
};

const DEFAULT_RIFLE_SPREAD: u32 = 15;
const DEFAULT_RIFLE_DAMAGE: u32 = 1;
const DEFAULT_RIFLE_COOLDOWN: u32 = 1000;

#[derive(Component)]
pub struct Rifle {
    pub spread: u32,
    pub damage: u32,
    pub cooldown: u32,
}

impl Default for Rifle {
    fn default() -> Self {
        Self {
            spread: DEFAULT_RIFLE_SPREAD,
            damage: DEFAULT_RIFLE_DAMAGE,
            cooldown: DEFAULT_RIFLE_COOLDOWN,
        }
    }
}

#[derive(Component)]
pub struct RifleCoolDown {
    pub timer: Timer,
}

impl Default for RifleCoolDown {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_millis(DEFAULT_RIFLE_COOLDOWN as u64),
                TimerMode::Repeating,
            ),
        }
    }
}

#[derive(Bundle)]
pub struct RifleBundle {
    pub rifle: Rifle,
    pub cooldown: RifleCoolDown,
}

impl Default for RifleBundle {
    fn default() -> Self {
        Self {
            rifle: default(),
            cooldown: default(),
        }
    }
}

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn fire_rifle(
    mut rifle_cooldown_query: Query<&mut RifleCoolDown>,
    mut player_rifle_target_query: Query<
        (&HasTarget, &Position, &Rifle),
        (With<Player>, Without<Projectile>),
    >,
    infected_query: Query<&Infected>,
    mut projectile_spawner: ProjectileSpawner,
    time: Res<Time>,
) {
    if !rifle_cooldown_query.is_empty() {
        let mut cooldown_timer = rifle_cooldown_query.get_single_mut().unwrap();
        cooldown_timer.timer.tick(time.delta());
        if !infected_query.is_empty() && cooldown_timer.timer.finished() {
            for (player_has_target, player_position, rifle) in &mut player_rifle_target_query {
                let spread = define_spread(
                    player_position.0,
                    player_has_target.target_position,
                    rifle.spread,
                );
                projectile_spawner.spawn_rifle_projectile(
                    player_position.0,
                    spread,
                    AimType::Random,
                )
            }
        }
    }
}
