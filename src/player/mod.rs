use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Position;

use crate::{global::*, mob::Mob};

use self::input::{PlayerAimSwap, PlayerWalk};

pub mod input;
pub mod spawner;

pub const DEFAULT_PLAYER_SIZE: f32 = 10.;
pub const DEFAULT_PLAYER_HIT_POINTS: i32 = 100;
pub const DEFAULT_PLAYER_DEFENSE: i32 = 1;
pub const DEFAULT_PLAYER_ATTACK_SPEED: f32 = 1000.;
pub const DEFAULT_PLAYER_MOVEMENT_SPEED: f32 = 3.;
pub const DEFAULT_PLAYER_ANTI_MOB_SPAWN_SIZE: f32 = 200.;
pub const DEFAULT_PLAYER_INVULNERABILITY: f64 = 1.;
pub const DEFAULT_PLAYER_AIM_TYPE: AimType = AimType::Closest;

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

pub fn move_player(
    mut events: EventReader<PlayerWalk>,
    mut query: Query<&mut Position, With<Player>>,
) {
    for player_walk_event in events.iter() {
        let mut player_position = query.single_mut();
        let direction_vec2: Vec2 = player_walk_event.direction.into();
        player_position.0 += direction_vec2 * DEFAULT_PLAYER_MOVEMENT_SPEED;
    }
}

pub fn swap_player_aim(
    mut player_aim_swap_events: EventReader<PlayerAimSwap>,
    mut aim_query: Query<&mut AimType, With<Player>>,
) {
    for _ in player_aim_swap_events.iter() {
        for mut aimtype in &mut aim_query {
            let next_aim = aimtype.next();
            *aimtype = next_aim;
        }
    }
}

#[derive(Event)]
pub struct PlayerAttackSpeedChange {}

pub fn update_player_attack_timer(
    mut commands: Commands,
    mut player_attack_speed_change_events: EventReader<PlayerAttackSpeedChange>,
    attack_speed_query: Query<&mut Stats, (With<Player>, Without<Mob>)>,
    mut timer_query: Query<Entity, (With<AttackTimer>, With<Player>)>,
) {
    for _ in player_attack_speed_change_events.iter() {
        for entity in &mut timer_query {
            println!(
                "this many player stats : {}",
                attack_speed_query.iter().count()
            );
            for player_stats in attack_speed_query.iter() {
                println!("player with timer!");
                let updated_attack_timer = AttackTimer {
                    timer: Timer::new(
                        Duration::from_millis(player_stats.attack_speed as u64),
                        TimerMode::Repeating,
                    ),
                };
                println!(
                    "adding timer with attack speed of {}",
                    updated_attack_timer.timer.duration().as_millis()
                );
                commands.entity(entity).remove::<AttackTimer>();
                commands.entity(entity).insert(updated_attack_timer);
            }
        }
    }
}
