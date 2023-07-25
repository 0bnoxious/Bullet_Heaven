use bevy::prelude::*;

use crate::{
    global::*, mob::infected::Infected, projectile::projectile_spawner::PlayerProjectileSpawner,
};

pub mod player_spawner;

pub const PLAYER_SIZE: f32 = 10.;
pub const ATTACK_SPEED: u64 = 10;
pub const PLAYER_SPEED: f32 = 3.;

#[derive(Component)]
pub struct Player {
    pub aim_type: AimType,
}

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
