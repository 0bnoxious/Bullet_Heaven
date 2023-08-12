use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Position;

use crate::global::{AimType, Stats};

use super::{input::*, Player};

pub fn move_player(
    mut events: EventReader<PlayerWalk>,
    mut player_query: Query<(&mut Position, &Stats), With<Player>>,
) {
    for player_walk_event in events.iter() {
        for (mut player_position, player_stats) in &mut player_query {
            let direction_vec2: Vec2 = player_walk_event.direction.into();
            player_position.0 += direction_vec2.normalize() * player_stats.movement_speed as f32;
        }
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
