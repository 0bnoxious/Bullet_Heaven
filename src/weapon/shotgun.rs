use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    global::AimType,
    mob::infected::Infected,
    player::{self, AttackTimer, Player},
    projectile::projectile_spawner::ProjectileSpawner,
    targeting::{define_spread, HasTarget},
};

pub const DEFAULT_SHOTGUN_BULLET_COUNT: i32 = 2;
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
    mut shotgun_target_query: Query<(&Shotgun, &HasTarget, &AimType), With<Player>>,
    player_position_query: Query<&Position, With<Player>>,
    infected_position_query: Query<&Infected>,
    mut projectile_spawner: ProjectileSpawner,
    time: Res<Time>,
) {
    if !infected_position_query.is_empty() {
        let mut attack_timer = attack_timer_query.get_single_mut().unwrap();
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.finished() {
            let player_pos = player_position_query.single().0;

            for (shotgun, target, aim_type) in &mut shotgun_target_query {
                println!("target from fireshotgun : {}", target.target_position);
                for _ in 0..shotgun.bullet_count {
                    //println!("beforespread : {test:?}");
                    let projectile_spread_direction =
                        define_spread(player_pos, target.target_position, shotgun.spread);
                    //println!("after spread : {projectile_spread_direction:?}");

                    projectile_spawner.spawn_shotgun_projectile(
                        player_pos,
                        projectile_spread_direction,
                        aim_type.clone(),
                    )
                }
            }
        }
    }
}
