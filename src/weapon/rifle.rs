use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Position;

use crate::{
    mob::infected::Infected,
    player::*,
    projectile::{spawner::ProjectileSpawner, Projectile},
    targeting::{define_spread, HasTarget},
};

use super::{Weapon, WeaponCoolDown};

#[derive(Component)]
pub struct Rifle;

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn fire_rifle(
    mut rifle_query: Query<(&Weapon, &mut WeaponCoolDown), With<Rifle>>,
    mut target_query: Query<(&HasTarget, &Position), (With<Player>, Without<Projectile>)>,
    infected_position_query: Query<&Infected>,
    mut projectile_spawner: ProjectileSpawner,
    time: Res<Time>,
) {
    if !rifle_query.is_empty() {
        for (rifle, mut rifle_cooldown) in &mut rifle_query {
            rifle_cooldown.timer.tick(time.delta());
            if !infected_position_query.is_empty() && rifle_cooldown.timer.finished() {
                for (player_has_target, player_position) in &mut target_query {
                    for _ in 0..rifle.bullet_count {
                        let spread = define_spread(
                            player_position.0,
                            player_has_target.target_position,
                            rifle.spread,
                        );
                        projectile_spawner.spawn_rifle_projectile(
                            player_position.0,
                            spread,
                            rifle.aim_type,
                        )
                    }
                }
            }
        }
    }
}
