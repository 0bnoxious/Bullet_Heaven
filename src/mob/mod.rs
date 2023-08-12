use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::global::*;
use crate::player::Player;
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
