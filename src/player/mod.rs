use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Position;

use crate::{
    global::*,
    mob::Mob,
    weapon::{
        rifle::{Rifle, RifleCoolDown},
        shotgun::{self, ShotGunCoolDown, Shotgun},
    },
};

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
pub struct PlayerShotGunCoolDownChange {}

pub fn update_player_shotgun_cooldown(
    mut commands: Commands,
    mut player_shotgun_cooldown_change_events: EventReader<PlayerShotGunCoolDownChange>,
    shotgun_query: Query<&mut Shotgun, With<Player>>,
    mut timer_query: Query<Entity, (With<ShotGunCoolDown>, With<Player>)>,
) {
    for _ in player_shotgun_cooldown_change_events.iter() {
        for entity in &mut timer_query {
            for shotgun in shotgun_query.iter() {
                let updated_attack_timer = ShotGunCoolDown {
                    timer: Timer::new(
                        Duration::from_millis(shotgun.fire_rate as u64),
                        TimerMode::Repeating,
                    ),
                };
                commands.entity(entity).remove::<ShotGunCoolDown>();
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
                        Duration::from_millis(rifle.fire_rate as u64),
                        TimerMode::Repeating,
                    ),
                };
                commands.entity(entity).remove::<RifleCoolDown>();
                commands.entity(entity).insert(updated_rifle_timer);
            }
        }
    }
}
