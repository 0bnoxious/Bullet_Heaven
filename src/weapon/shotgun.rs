use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{
    global::AimType,
    mob::infected::Infected,
    player::{AttackTimer, Player},
    projectile::{projectile_spawner::ProjectileSpawner, Projectile},
    targeting::{define_spread, HasTarget},
};

pub const DEFAULT_SHOTGUN_BULLET_COUNT: i32 = 20;
pub const DEFAULT_SHOTGUN_SPREAD: f32 = 15.;
pub const DEFAULT_SHOTGUN_DAMAGE: f64 = 1.;
pub const DEFAULT_SHOTGUN_FIRE_RATE: f64 = 1000.;
pub const DEFAULT_SHOTGUN_RANGE: f64 = 100.;

#[derive(Component)]
pub struct Shotgun {
    pub bullet_count: i32,
    pub spread: f32,
    pub damage: f64,
    pub fire_rate: f64,
    pub range: f64,
}

impl Default for Shotgun {
    fn default() -> Self {
        Self {
            bullet_count: DEFAULT_SHOTGUN_BULLET_COUNT,
            spread: DEFAULT_SHOTGUN_SPREAD,
            range: DEFAULT_SHOTGUN_RANGE,
            damage: DEFAULT_SHOTGUN_DAMAGE,
            fire_rate: DEFAULT_SHOTGUN_FIRE_RATE,
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn fire_shotgun(
    mut attack_timer_query: Query<&mut AttackTimer>,
    mut query: Query<(&HasTarget, &Position, &Shotgun), (With<Player>, Without<Projectile>)>,
    infected_position_query: Query<&Infected>,
    mut projectile_spawner: ProjectileSpawner,
    time: Res<Time>,
) {
    if !infected_position_query.is_empty() {
        let mut attack_timer = attack_timer_query.get_single_mut().unwrap();
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.finished() {
            for (player_has_target, player_position, shotgun) in &mut query {
                for _ in 0..shotgun.bullet_count {
                    let spread = define_spread(
                        player_position.0,
                        player_has_target.target_position,
                        shotgun.spread,
                    )
                    .normalize();
                    projectile_spawner.spawn_shotgun_projectile(
                        player_position.0,
                        spread,
                        AimType::Closest,
                    )
                }
            }
        }
    }
}
