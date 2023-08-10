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

const DEFAULT_RIFLE_SPREAD: f32 = 15.;
const DEFAULT_RIFLE_DAMAGE: f64 = 1.;
const DEFAULT_RIFLE_FIRE_RATE: f64 = 1000.;
const DEFAULT_RIFLE_RANGE: f64 = 100.;

#[derive(Component)]
pub struct Rifle {
    pub spread: f32,
    pub damage: f64,
    pub fire_rate: f64,
    pub range: f64,
}

impl Default for Rifle {
    fn default() -> Self {
        Self {
            spread: DEFAULT_RIFLE_SPREAD,
            range: DEFAULT_RIFLE_RANGE,
            damage: DEFAULT_RIFLE_DAMAGE,
            fire_rate: DEFAULT_RIFLE_FIRE_RATE,
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
                Duration::from_millis(DEFAULT_RIFLE_FIRE_RATE as u64),
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
        let mut attack_timer = rifle_cooldown_query.get_single_mut().unwrap();
        attack_timer.timer.tick(time.delta());
        if !infected_query.is_empty() && attack_timer.timer.finished() {
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
