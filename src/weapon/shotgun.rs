use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    global::{Layer, Target},
    mob::infected::Infected,
    player::{AttackTimer, Player},
    projectile::{
        projectile_spawner::{self, ProjectileSpawner},
        Projectile, ProjectileTimer, PROJECTILE_LIFE_SPAN, PROJECTILE_SIZE, PROJECTILE_SPEED,
    },
};

use super::{default_weapon, ClosestTarget, Weapon};

pub const DEFAULT_SHOTGUN_BULLET_COUNT: i32 = 8;
pub const DEFAULT_SHOTGUN_SPREAD: f64 = 1.5;
pub const DEFAULT_SHOTGUN_DAMAGE: f64 = 1.;
pub const DEFAULT_SHOTGUN_FIRE_RATE: f64 = 1000.;
pub const DEFAULT_SHOTGUN_RANGE: f64 = 100.;

#[derive(Component)]
pub struct Shotgun {
    pub bullet_count: i32,
    pub spread: f64,
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

pub fn define_spread(origin: Vec2, direction: Vec2, spread: f64) -> Vec2 {
    let mut rng = rand::thread_rng();
    //determine deviation from target using a bell curve type distribution
    let deviation = rng.gen_range(0.0..spread) + rng.gen_range(0.0..spread) - spread;
    println!("deviation : {deviation:?}");
    //rotate the target vector by the deviation
    let (y, x) = (deviation + std::f64::consts::PI / 2.0).sin_cos();
    let deviation_as_vec = Vec2::new(x as f32, y as f32);
    println!("deviation as vec: {deviation_as_vec:?}");

    (direction - origin).normalize() + deviation_as_vec
}

#[allow(clippy::too_many_arguments)]
pub fn fire_shotgun(
    mut attack_timer_query: Query<&mut AttackTimer>,
    player_position_query: Query<&Position, With<Player>>,
    shotgun_query: Query<&Shotgun>,
    time: Res<Time>,
    infected_query: Query<&Position, With<Infected>>,
    mut closest: ClosestTarget,
    mut projectile_spawner: ProjectileSpawner,
) {
    if infected_query.iter().count() > 0 {
        let mut attack_timer = attack_timer_query.get_single_mut().unwrap();
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.finished() {
            let player_pos = player_position_query.single();
            let bullet_count = shotgun_query.single().bullet_count;

            for _ in 0..bullet_count {
                let spread2 = shotgun_query.single().spread;
                let spread_vec =
                    define_spread(player_pos.0, closest.infected().position.0, spread2);
                let t = Target {
                    position: { Position(spread_vec.normalize() * PROJECTILE_SPEED) },
                };

                //println!("base spread : {spread2:?}");
                //println!("bullet spread : ({}, {})", t.position.x, t.position.y);
                //println!("player pos : {player_pos:?}");

                projectile_spawner.spawn_shotgun_projectile(*player_pos, t)
            }
        }
    }
}
