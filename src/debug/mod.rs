use bevy::prelude::*;
use bevy_xpbd_2d::prelude::LinearVelocity;

use crate::{global::Stats, player::Player, projectile::Projectile, targeting::HasTarget};

pub mod egui;
pub mod gizmo;

#[allow(clippy::type_complexity)]
pub fn move_projectile_to_target(
    mut projectile_velocity_query: Query<&mut LinearVelocity, With<Projectile>>,
    projectile_target_query: Query<&HasTarget, With<Projectile>>,
) {
    for mut velocity in &mut projectile_velocity_query {
        if velocity.0 == Vec2::ZERO {
            for target in projectile_target_query.iter() {
                velocity.0 = target.target_position * 500.
            }
        }
    }
}

pub fn log_player_hitpoint(player_stats_query: Query<&Stats, With<Player>>) {
    for player_stats in player_stats_query.iter() {
        println!("Player HP: {}", player_stats.hit_points);
    }
}
