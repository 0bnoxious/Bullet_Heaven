use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::CollisionStarted;

use crate::{
    global::*,
    mob::infected::Infected,
    projectile::Damage,
    weapon::{
        rifle::{Rifle, RifleCoolDown},
        shotgun::{Shotgun, ShotgunCoolDown},
    },
};

pub mod action;
pub mod input;
pub mod spawner;

pub const DEFAULT_PLAYER_SIZE: u32 = 10;
pub const DEFAULT_PLAYER_HIT_POINTS: i32 = 100;
pub const DEFAULT_PLAYER_DEFENSE: i32 = 1;
pub const DEFAULT_PLAYER_ATTACK_SPEED: u32 = 1000;
pub const DEFAULT_PLAYER_MOVEMENT_SPEED: u32 = 3;
pub const DEFAULT_PLAYER_ANTI_MOB_SPAWN_SIZE: u32 = 200;
pub const DEFAULT_PLAYER_INVULNERABILITY: u32 = 1;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}

pub fn default_player_stats() -> Stats {
    Stats {
        hit_points: DEFAULT_PLAYER_HIT_POINTS,
        movement_speed: DEFAULT_PLAYER_MOVEMENT_SPEED,
        attack_speed: DEFAULT_PLAYER_ATTACK_SPEED,
        defense: DEFAULT_PLAYER_DEFENSE,
        damage: 1,
    }
}

#[derive(Event)]
pub struct PlayerShotGunCoolDownChange {}

pub fn update_player_shotgun_cooldown(
    mut commands: Commands,
    mut player_shotgun_cooldown_change_events: EventReader<PlayerShotGunCoolDownChange>,
    shotgun_query: Query<&mut Shotgun, With<Player>>,
    mut timer_query: Query<Entity, (With<ShotgunCoolDown>, With<Player>)>,
) {
    for _ in player_shotgun_cooldown_change_events.iter() {
        for entity in &mut timer_query {
            for shotgun in shotgun_query.iter() {
                println!("new shotgun cooldown value {}", shotgun.cooldown);
                let updated_attack_timer = ShotgunCoolDown {
                    timer: Timer::new(
                        Duration::from_millis(shotgun.cooldown as u64),
                        TimerMode::Repeating,
                    ),
                };
                commands.entity(entity).remove::<ShotgunCoolDown>();
                commands.entity(entity).insert(updated_attack_timer);
            }
        }
    }
}

#[derive(Event)]
pub struct PlayerRifleCoolDownChange {}

pub fn update_player_rifle_cooldown(
    mut commands: Commands,
    mut player_attack_speed_change_events: EventReader<PlayerRifleCoolDownChange>,
    rifle_query: Query<&mut Rifle, With<Player>>,
    mut timer_query: Query<Entity, (With<RifleCoolDown>, With<Player>)>,
) {
    for _ in player_attack_speed_change_events.iter() {
        for entity in &mut timer_query {
            for rifle in rifle_query.iter() {
                let updated_rifle_timer = RifleCoolDown {
                    timer: Timer::new(
                        Duration::from_millis(rifle.cooldown as u64),
                        TimerMode::Repeating,
                    ),
                };
                commands.entity(entity).remove::<RifleCoolDown>();
                commands.entity(entity).insert(updated_rifle_timer);
            }
        }
    }
}

pub fn player_damage(
    mut is_player_damage: Query<&mut Damage, With<Player>>,
    is_infected_stats: Query<&Stats, With<Infected>>,
    mut events: EventReader<CollisionStarted>,
) {
    let mut collide = |entity_a: &Entity, entity_b: &Entity| -> bool {
        if is_infected_stats.get(*entity_a).is_ok() {
            let infected_dmg_stat = is_infected_stats.get(*entity_a).unwrap().damage;
            if is_player_damage.get(*entity_b).is_ok() {
                for mut player_damage in &mut is_player_damage {
                    player_damage.instances.push(infected_dmg_stat);
                    //println!("taking damage! : {}", infected_dmg_stat);
                }
                return true;
            }
        }
        false
    };

    // if entity a is not a player, flip'em.
    for CollisionStarted(entity_a, entity_b) in events.iter() {
        if !collide(entity_a, entity_b) {
            collide(entity_b, entity_a);
        }
    }
}
