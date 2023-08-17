use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::CollisionStarted;

use crate::global::*;
use crate::player::{AttackTimer, Player};
use crate::projectile::Damage;

use self::infected::*;

pub mod infected;
pub mod spawner;

#[derive(Component)]
pub struct Mob;

/*#[derive(Resource)]
pub struct RandomDirectionTimer {
    pub timer: Timer,
}*/

// #[allow(clippy::type_complexity)]
// pub fn confuse_mobs(
//     mut velocity_query: Query<
//         &mut LinearVelocity,
//         (Without<Projectile>, Without<Player>, Without<Sensor>),
//     >,
//     time: Res<Time>,
//     mut timer_res: ResMut<RandomDirectionTimer>,
// ) {
//     timer_res.timer.tick(time.delta());

//     let mut rng = rand::thread_rng();
//     for mut velocity in &mut velocity_query {
//         if timer_res.timer.just_finished() {
//             let new_velocity = random_velocity(&mut rng);
//             velocity.x = new_velocity.x * HEALTHY_MOVEMENT_SPEED;
//             velocity.y = new_velocity.y * HEALTHY_MOVEMENT_SPEED;
//         }
//     }
// }

pub fn attack_player(
    mut is_player_damage: Query<&mut Damage, With<Player>>,
    is_infected_stats: Query<&Stats, With<Infected>>,
    mut is_infected_attack_timer: Query<&mut AttackTimer, With<Infected>>,
    mut events: EventReader<CollisionStarted>,
    time: Res<Time>,
) {
    let mut collide = |entity_a: &Entity, entity_b: &Entity| -> bool {
        if is_infected_stats.get(*entity_a).is_ok() {
            let mut infected_attack_timer = is_infected_attack_timer.get_mut(*entity_a).unwrap();
            infected_attack_timer.timer.tick(time.delta());

            //infected_attack_timer.timer.tick(time.delta());
            if infected_attack_timer.timer.finished() {
                let infected_dmg_stat = is_infected_stats.get(*entity_a).unwrap().damage;
                if is_player_damage.get(*entity_b).is_ok() {
                    for mut player_damage in &mut is_player_damage {
                        player_damage.instances.push(infected_dmg_stat);
                        println!("taking damage! : {}", infected_dmg_stat);
                    }
                    return true;
                }
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
