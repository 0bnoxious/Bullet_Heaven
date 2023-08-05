use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{
    global::*,
    mob::infected::Infected,
    player::Player,
    targeting::{ClosestTarget, HasTarget},
};

pub mod projectile_spawner;

pub const PROJECTILE_SIZE: f32 = 8.;
pub const PROJECTILE_SPEED: f32 = 800.;
pub const PROJECTILE_DAMAGE: i32 = 1;
pub const PROJECTILE_LIFE_SPAN: u64 = 1;

#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Component)]
pub struct Damage {
    pub instances: Vec<i32>,
}

#[derive(Component)]
pub struct ProjectileTimer {
    pub timer: Timer,
}

#[derive(Component, Debug)]
pub enum ProjectileType {
    Bullet,
    //Explosive,
    //Lazer,
    //Orbiting,
    //Lobbing,
}

#[allow(clippy::type_complexity)]
pub fn move_shotgun_projectile(
    mut projectile_query: Query<
        (
            &Position,
            &mut LinearVelocity,
            &mut Rotation,
            &mut HasTarget,
        ),
        With<Projectile>,
    >,
    player_query: Query<&Position, With<Player>>,
    projectile_aim_query: Query<&AimType, With<Projectile>>,
    mut closest_target: ClosestTarget,
) {
    for projectile_aim in projectile_aim_query.iter() {
        match projectile_aim {
            AimType::Random => {
                let mut rng = rand::thread_rng();
                //let random_velocity = random_velocity(&mut rng);

                for (_, mut vel, _, target) in &mut projectile_query {
                    vel.0 = target.target_position * random_velocity(&mut rng).truncate();
                }
            }

            // aim the position of the mouse at spawn
            //AimType::Mouse => unimplemented!(),

            // constantly aim the mouse position
            //AimType::HomingMouse => unimplemented!(),

            // aim the position of the closest target at spawn
            AimType::Closest => {
                for (_, mut projectile_velocity, mut projectile_rotation, projectile_target) in
                    &mut projectile_query
                {
                    // set the velocity toward closest target at spawn
                    if projectile_velocity.x == 0. && projectile_velocity.y == 0. {
                        /*let player_position = Vec3::new(
                            player_query.get_single().unwrap().x,
                            player_query.get_single().unwrap().y,
                            0.,
                        );

                        // Cast Projectile target position as Vec3 for quat rotation
                        // get the vector from the projectile to the closest infected.
                        let closest = projectile_target.target_position;
                        let to_closest = Vec3::new(closest.x, closest.y, 0.) - player_position;

                        // get the quaternion to rotate from the initial projectile facing direction to the direction
                        // facing the closest infected
                        let rotate_to_infected = Quat::from_rotation_arc(Vec3::Y, to_closest);*/
                        // rotate the projectile to face the closest infected
                        //*projectile_rotation = Rotation::from(rotate_to_infected);

                        projectile_velocity.x =
                            projectile_target.target_position.x * PROJECTILE_SPEED;
                        projectile_velocity.y =
                            projectile_target.target_position.y * PROJECTILE_SPEED;
                    }
                }
            }

            // constantly aim the closest target
            AimType::HomingClosest => {
                for (projectile_position, mut projectile_velocity, mut projectile_rotation, _) in
                    &mut projectile_query
                {
                    // Cast Projectile target position as Vec3 for quat rotation
                    let closest = closest_target.infected();
                    let projectile_target_vec3 = Vec3::new(closest.x, closest.y, 0.);

                    // get the vector from the projectile to the closest infected and normalise it.
                    let to_closest = (projectile_target_vec3
                        - Vec3 {
                            x: projectile_position.x,
                            y: projectile_position.y,
                            z: 0.,
                        })
                    .normalize();

                    // get the quaternion to rotate from the initial projectile facing direction to the direction
                    // facing the closest infected
                    let rotate_to_infected = Quat::from_rotation_arc(Vec3::Y, to_closest);

                    // rotate the projectile to face the closest infected
                    *projectile_rotation = Rotation::from(rotate_to_infected);
                    projectile_velocity.x = to_closest.x * PROJECTILE_SPEED;
                    projectile_velocity.y = to_closest.y * PROJECTILE_SPEED;
                }
            }
        }
    }
}

pub fn handle_projectile_collision(
    mut commands: Commands,
    mut infected_query: Query<&mut Damage, With<Infected>>,
    mut events: EventReader<CollisionStarted>,
    is_projectile: Query<&Projectile>,
) {
    let mut collide = |entity_a: &Entity, entity_b: &Entity| -> bool {
        if is_projectile.get(*entity_a).is_ok() {
            // get the target's damage stack
            if let Ok(mut damage) = infected_query.get_mut(*entity_b) {
                // add the projectile damage to the damage stack
                damage.instances.push(PROJECTILE_DAMAGE);
                // delete projectile after contact
                commands.entity(*entity_a).insert(Dead);
                return true;
            }
        }
        false
    };

    // if entity a is not a projectile, flip'em.
    for CollisionStarted(entity_a, entity_b) in events.iter() {
        if !collide(entity_a, entity_b) {
            collide(entity_b, entity_a);
        }
    }
}

/*#[allow(clippy::type_complexity)]
pub fn move_projectile(
    mut projectile_query: Query<
        (
            &Position,
            &mut LinearVelocity,
            &mut Rotation,
            &mut HasTarget,
        ),
        With<Projectile>,
    >,
    player_query: Query<&Position, With<Player>>,
    projectile_aim_query: Query<&AimType, With<Projectile>>,
    mut closest_target: ClosestTarget,
) {
    for projectile_aim in projectile_aim_query.iter() {
        match projectile_aim {
            AimType::Random => {
                let mut rng = rand::thread_rng();
                let new_velocity = random_velocity(&mut rng);
                for (_, mut projectile_velocity, _, _) in &mut projectile_query {
                    if projectile_velocity.x == 0. && projectile_velocity.y == 0. {
                        projectile_velocity.x = new_velocity.x * PROJECTILE_SPEED;
                        projectile_velocity.y = new_velocity.y * PROJECTILE_SPEED;
                    }
                }
            }

            // aim the position of the mouse at spawn
            //AimType::Mouse => unimplemented!(),

            // constantly aim the mouse position
            //AimType::HomingMouse => unimplemented!(),

            // aim the position of the closest target at spawn
            AimType::Closest => {
                for (_, mut projectile_velocity, mut projectile_rotation, _) in
                    &mut projectile_query
                {
                    // set the velocity toward closest target at spawn
                    if projectile_velocity.x == 0. && projectile_velocity.y == 0. {
                        let player_position = Vec3::new(
                            player_query.get_single().unwrap().x,
                            player_query.get_single().unwrap().y,
                            0.,
                        );

                        // Cast Projectile target position as Vec3 for quat rotation
                        let closest = closest_target.infected();
                        let closest_vec3 = Vec3::new(closest.x, closest.y, 0.);

                        // get the vector from the projectile to the closest infected.
                        let to_closest = (closest_vec3 - player_position).normalize();

                        // get the quaternion to rotate from the initial projectile facing direction to the direction
                        // facing the closest infected
                        let rotate_to_infected = Quat::from_rotation_arc(Vec3::Y, to_closest);

                        // rotate the projectile to face the closest infected
                        *projectile_rotation = Rotation::from(rotate_to_infected);
                        projectile_velocity.x = to_closest.x * PROJECTILE_SPEED;
                        projectile_velocity.y = to_closest.y * PROJECTILE_SPEED;
                    }
                }
            }

            // constantly aim the closest target
            AimType::HomingClosest => {
                for (projectile_position, mut projectile_velocity, mut projectile_rotation, _) in
                    &mut projectile_query
                {
                    // Cast Projectile target position as Vec3 for quat rotation
                    let closest = closest_target.infected();
                    let projectile_target_vec3 = Vec3::new(closest.x, closest.y, 0.);

                    // get the vector from the projectile to the closest infected and normalise it.
                    let to_closest = (projectile_target_vec3
                        - Vec3 {
                            x: projectile_position.x,
                            y: projectile_position.y,
                            z: 0.,
                        })
                    .normalize();

                    // get the quaternion to rotate from the initial projectile facing direction to the direction
                    // facing the closest infected
                    let rotate_to_infected = Quat::from_rotation_arc(Vec3::Y, to_closest);

                    // rotate the projectile to face the closest infected
                    *projectile_rotation = Rotation::from(rotate_to_infected);
                    projectile_velocity.x = to_closest.x * PROJECTILE_SPEED;
                    projectile_velocity.y = to_closest.y * PROJECTILE_SPEED;
                }
            }
        }
    }
}*/
