use ::bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    global::{random_direction, AimType},
    player::Player,
    targeting::{ClosestTarget, HasTarget},
};

use super::{
    spawner::{FromRifle, FromShotgun},
    Homing, Projectile, PROJECTILE_SPEED,
};

const PROJECTILE_SPEED_VARIANCE_PERCENTAGE: f32 = 15.;

#[allow(clippy::type_complexity)]
pub fn move_rifle_projectile(
    mut projectile_query: Query<
        (&mut LinearVelocity, &mut Rotation, &mut HasTarget, &AimType),
        (With<Projectile>, With<FromRifle>),
    >,
    player_query: Query<&Position, With<Player>>,
) {
    for (mut vel, mut rotation, target, projectile_aim) in &mut projectile_query {
        match projectile_aim {
            AimType::Random => {
                let mut rng = rand::thread_rng();

                if vel.0 == Vec2::ZERO {
                    vel.0 = target.target_position
                        * random_direction(&mut rng).truncate()
                        * PROJECTILE_SPEED;
                }
            }

            // aim the position of the closest target at spawn
            AimType::Closest => {
                // set the velocity toward closest target at spawn
                if vel.0 == Vec2::ZERO {
                    let player_position = Vec3::new(
                        player_query.get_single().unwrap().x,
                        player_query.get_single().unwrap().y,
                        0.,
                    );

                    // Cast Projectile target position as Vec3 for quat rotation
                    // get the vector from the projectile to the closest infected.
                    let closest = target.target_position;
                    let to_closest = Vec3::new(closest.x, closest.y, 0.) - player_position;

                    // get the quaternion to rotate from the initial projectile facing direction to the direction
                    // facing the closest infected
                    let rotate_to_infected = Quat::from_rotation_arc(Vec3::Y, to_closest);
                    //rotate the projectile to face the closest infected
                    *rotation = Rotation::from(rotate_to_infected);

                    vel.0 = target.target_position * apply_speed_variance(PROJECTILE_SPEED);
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn move_shotgun_projectile(
    mut projectile_query: Query<
        (&AimType, &mut LinearVelocity, &mut Rotation, &mut HasTarget),
        (With<Projectile>, With<FromShotgun>),
    >,
    player_query: Query<&Position, With<Player>>,
) {
    for (projectile_aim, mut vel, mut projectile_rotation, target) in &mut projectile_query {
        match projectile_aim {
            AimType::Random => {
                let mut rng = rand::thread_rng();
                vel.0 = target.target_position * random_direction(&mut rng).truncate();
            }

            // aim the position of the closest target at spawn
            AimType::Closest => {
                // set the velocity toward closest target at spawn
                if vel.0 == Vec2::ZERO {
                    let player_position = Vec3::new(
                        player_query.get_single().unwrap().x,
                        player_query.get_single().unwrap().y,
                        0.,
                    );

                    // Cast Projectile target position as Vec3 for quat rotation
                    // get the vector from the projectile to the closest infected.
                    let closest = target.target_position;
                    let to_closest = Vec3::new(closest.x, closest.y, 0.) - player_position;

                    // get the quaternion to rotate from the initial projectile facing direction to the direction
                    // facing the closest infected
                    let rotate_to_infected = Quat::from_rotation_arc(Vec3::Y, to_closest);
                    //rotate the projectile to face the closest infected
                    *projectile_rotation = Rotation::from(rotate_to_infected);

                    vel.0 = target.target_position * apply_speed_variance(PROJECTILE_SPEED);
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn move_homing_projectile(
    mut projectile_query: Query<
        (
            &Position,
            &mut LinearVelocity,
            &mut Rotation,
            &mut HasTarget,
        ),
        (With<Projectile>, With<Homing>),
    >,
    mut closest: ClosestTarget,
) {
    for (projectile_position, mut projectile_velocity, mut projectile_rotation, _) in
        &mut projectile_query
    {
        // Cast Projectile target position as Vec3 for quat rotation
        let closest = closest.infected();
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
        projectile_velocity.x = to_closest.x * apply_speed_variance(PROJECTILE_SPEED);
        projectile_velocity.y = to_closest.y * apply_speed_variance(PROJECTILE_SPEED);
    }
}

pub fn apply_speed_variance(initial_speed: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let minimum_speed = initial_speed * ((100. - PROJECTILE_SPEED_VARIANCE_PERCENTAGE) / 100.);
    let maximum_speed = initial_speed * ((100. + PROJECTILE_SPEED_VARIANCE_PERCENTAGE) / 100.);
    //determine deviation from target using a bell curve type distribution

    rng.gen_range(minimum_speed..maximum_speed) + rng.gen_range(minimum_speed..maximum_speed)
        - PROJECTILE_SPEED_VARIANCE_PERCENTAGE * 2.
}
