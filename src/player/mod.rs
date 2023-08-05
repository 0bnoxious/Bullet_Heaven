use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Position;

use crate::global::*;

use self::player_input::{PlayerAimSwap, PlayerWalk};

pub mod player_input;
pub mod player_spawner;

pub const PLAYER_SIZE: f32 = 10.;
pub const PLAYER_HHIT_POINTS: f32 = 100.;
pub const ATTACK_SPEED: u64 = 10;
pub const PLAYER_SPEED: f32 = 3.;
pub const PLAYER_ANTI_MOB_SPAWN_SIZE: f32 = 200.;
pub const PLAYER_INVULNERABILITY: f64 = 1.;
pub const PLAYER_AIM_TYPE: AimType = AimType::Closest;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
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

/*pub fn player_attack(
    time: Res<Time>,
    mut attack_timer_query: Query<&mut AttackTimer>,
    infected_query: Query<(), With<Infected>>,
    player_pos_query: Query<&Position, With<Player>>,
    player_target_query: Query<&HasTarget, With<Player>>,
    mut projectile_spawner: ProjectileSpawner,
) {
    if infected_query.iter().count() > 0 {
        let mut attack_timer = attack_timer_query.get_single_mut().unwrap();
        attack_timer.timer.tick(time.delta());
        if attack_timer.timer.finished() {
            for player_target in player_target_query.iter() {
                projectile_spawner
                    .spawn_projectile(player_pos_query.single().0, player_target.target_position);
            }
        }
    }
}*/

/*pub fn update_player_target(
    player_target_query: Query<&mut Target, With<Player>>,
    mut target: ClosestTarget,
) {
    let mut target_position = player_target_query.single().position;
    target_position = target.infected().position;
}*/
