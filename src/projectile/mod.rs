use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{global::*, mob::infected::Infected, player::Player};

pub mod projectile_spawner;

pub const PROJECTILE_SIZE: f32 = 8.;
pub const PROJECTILE_SPEED: f32 = 300.;
pub const PROJECTILE_DAMAGE: i32 = 1;
pub const PROJECTILE_LIFE_SPAN: u64 = 2;

#[derive(Component)]
pub struct ProjectileTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Damage {
    pub instances: Vec<i32>,
}

#[derive(Component, Debug)]
pub struct Projectile;

#[allow(clippy::type_complexity)]
pub fn move_projectile(
    mut projectile_query: Query<
        (&Position, &mut LinearVelocity, &mut Rotation, &mut Closest),
        With<Projectile>,
    >,
    mut infected_query: Query<&Position, With<Infected>>,
    player_query: Query<&Position, With<Player>>,
    player_aim_query: Query<&AimType, With<Player>>,
) {
    let aim_type = player_aim_query.single();

    match aim_type {
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
            for (_, mut projectile_velocity, mut projectile_rotation, mut projectile_closest) in
                &mut projectile_query
            {
                // set the velocity toward closest target at spawn
                if projectile_velocity.x == 0. && projectile_velocity.y == 0. {
                    let mut current_closest_distance = f32::MAX;
                    let player_position = Vec3::new(
                        player_query.get_single().unwrap().x,
                        player_query.get_single().unwrap().y,
                        0.,
                    );

                    for infected_position in &mut infected_query {
                        let distance =
                            Vec2::distance(infected_position.0, player_position.truncate());

                        if distance < current_closest_distance {
                            current_closest_distance = distance;
                            projectile_closest.vec3 =
                                Vec3::new(infected_position.x, infected_position.y, 0.);
                        }
                    }

                    // get the vector from the projectile to the closest infected.
                    let to_closest = (projectile_closest.vec3 - player_position).normalize();

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
            for (
                projectile_position,
                mut projectile_velocity,
                mut projectile_rotation,
                mut projectile_closest_target,
            ) in &mut projectile_query
            {
                let mut closest_distance = f32::MAX;
                for infected_position in &mut infected_query.iter() {
                    // get the distance between infecteds and projectiles.
                    let distance = Vec2::distance(infected_position.0, projectile_position.0);

                    if distance < closest_distance {
                        closest_distance = distance;
                        // closest infected position as vec3.
                        projectile_closest_target.vec3 =
                            Vec3::new(infected_position.x, infected_position.y, 0.);
                    }
                }

                // get the vector from the projectile to the closest infected and normalise it.
                let to_closest = (projectile_closest_target.vec3
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
