use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Position;

use crate::{
    global::*, mob::infected::Infected, projectile::projectile_spawner::PlayerProjectileSpawner,
};

use self::player_input::{PlayerAimSwap, PlayerWalk};

pub mod player_input;
pub mod player_spawner;

pub const PLAYER_SIZE: f32 = 10.;
pub const ATTACK_SPEED: u64 = 10;
pub const BULLETS_PER_TICK: i32 = 1;
pub const PLAYER_SPEED: f32 = 3.;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}

pub fn player_attack(
    time: Res<Time>,
    mut attack_timer_query: Query<&mut AttackTimer>,
    infected_query: Query<(), With<Infected>>,
    mut player_counter: PlayerProjectileSpawner,
) {
    if infected_query.iter().count() > 0 {
        let mut attack_timer = attack_timer_query.get_single_mut().unwrap();
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.finished() {
            player_counter.spawn_projectile();
        }
    }
}

pub fn move_player(
    mut events: EventReader<PlayerWalk>,
    mut query: Query<&mut Position, With<Player>>,
) {
    for player_walk_event in events.iter() {
        let mut player_position = query.single_mut();
        let direction_vec2: Vec2 = player_walk_event.direction.into();
        player_position.0 += direction_vec2 * PLAYER_SPEED;
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
