use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    mob::infected::Infected,
    player::{AttackTimer, Player},
    projectile::{
        projectile_spawner::{self, ProjectileSpawner},
        Projectile, ProjectileTimer, PROJECTILE_LIFE_SPAN, PROJECTILE_SIZE, PROJECTILE_SPEED,
    },
    targeting::HasTarget,
};

use super::{default_weapon, Weapon};

pub const DEFAULT_SHOTGUN_BULLET_COUNT: i32 = 2;
pub const DEFAULT_SHOTGUN_SPREAD: f64 = 3.;
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
    let deviation_as_vec = Vec2::new(x as f32, y as f32).normalize();
    println!("deviation as vec: {deviation_as_vec:?}");

    (direction - origin).normalize() + deviation_as_vec
}

#[allow(clippy::too_many_arguments)]
pub fn fire_shotgun(
    mut attack_timer_query: Query<&mut AttackTimer>,
    mut shotgun_target_query: Query<(&Shotgun, &HasTarget)>,
    player_position_query: Query<&Position, With<Player>>,
    infected_position_query: Query<&Position, With<Infected>>,
    mut projectile_spawner: ProjectileSpawner,
    time: Res<Time>,
    //has_target_querry: Query<&HasTarget, With<Shotgun>>,
) {
    if infected_position_query.iter().count() > 0 {
        let mut attack_timer = attack_timer_query.get_single_mut().unwrap();
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.finished() {
            let player_pos = player_position_query.single().0;

            //println!("skoica {shotgun_target_query:?}");

            for (shotgun, target) in &mut shotgun_target_query {
                //let bullet_count = shotgun.bullet_count;
                //println!("shooting {bullet_count:?} bullets!");
                for _ in 0..shotgun.bullet_count {
                    let projectile_spread_direction =
                        define_spread(player_pos, target.target_position, shotgun.spread);

                    //println!("base spread : {spread2:?}");
                    //println!("bullet spread : ({}, {})", t.position.x, t.position.y);
                    //println!("player pos : {player_pos:?}");

                    projectile_spawner
                        .spawn_shotgun_projectile(player_pos, projectile_spread_direction)
                }
            }
        }
    }
}
