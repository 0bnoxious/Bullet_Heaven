use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{
    mob::infected::Infected,
    player::*,
    projectile::{spawner::ProjectileSpawner, Projectile},
    targeting::{define_spread, HasTarget},
};

use super::{Weapon, WeaponCoolDown};

#[derive(Component)]
pub struct Shotgun;

#[allow(clippy::type_complexity)]
#[allow(clippy::too_many_arguments)]
pub fn fire_shotgun(
    mut shotgun_query: Query<(&Weapon, &mut WeaponCoolDown), (With<Weapon>, With<Shotgun>)>,
    mut target_query: Query<(&HasTarget, &Position), (With<Player>, Without<Projectile>)>,
    infected_position_query: Query<&Infected>,
    mut projectile_spawner: ProjectileSpawner,
    time: Res<Time>,
) {
    if !shotgun_query.is_empty() {
        for (shotgun, mut shotgun_cooldown) in &mut shotgun_query {
            shotgun_cooldown.timer.tick(time.delta());
            if !infected_position_query.is_empty() && shotgun_cooldown.timer.finished() {
                for (player_has_target, player_position) in &mut target_query {
                    for _ in 0..shotgun.bullet_count {
                        let spread = define_spread(
                            player_position.0,
                            player_has_target.target_position,
                            shotgun.spread,
                        );
                        projectile_spawner.spawn_shotgun_projectile(
                            player_position.0,
                            spread,
                            shotgun.aim_type,
                        )
                    }
                }
            }
        }
    }
}
