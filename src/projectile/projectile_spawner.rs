use std::time::Duration;

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_xpbd_2d::prelude::*;

use crate::{
    global::*,
    mob::{infected::Infected, Stats, PERSONSIZE},
    player::player_spawner::*,
};

use super::{PROJECTILELIFESPAN, PROJECTILESIZE, PROJECTILESPEED};

#[derive(Component)]
pub struct ProjectileTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Projectile;

#[derive(SystemParam)]
pub struct PlayerProjectileSpawner<'w, 's> {
    commands: Commands<'w, 's>,
    players: Query<'w, 's, &'static Transform, With<Player>>,
}

impl<'w, 's> PlayerProjectileSpawner<'w, 's> {
    pub fn spawn_projectile(&mut self) {
        let player_position = self.players.single().translation;

        self.commands.spawn((
            Projectile,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: (Some(Vec2 {
                        x: PROJECTILESIZE,
                        y: PROJECTILESIZE,
                    })),
                    ..default()
                },
                transform: Transform::from_translation(Vec3 {
                    x: 10.,
                    y: 10.,
                    z: 0.0,
                }),
                ..default()
            },
            RigidBody::Kinematic,
            Position(Vec2::new(player_position.x, player_position.y)),
            Collider::cuboid(PROJECTILESIZE, PROJECTILESIZE),
            Closest {
                vec3: Vec3::new(0., 0., 0.),
            },
            ProjectileTimer {
                timer: Timer::new(Duration::from_secs(PROJECTILELIFESPAN), TimerMode::Once),
            },
        ));
    }
}

#[allow(clippy::type_complexity)]
pub fn move_projectile(
    mut projectile_query: Query<
        (&Position, &mut LinearVelocity, &mut Rotation, &mut Closest),
        With<Projectile>,
    >,
    mut infected_query: Query<&Position, With<Infected>>,
    player_query: Query<&Position, With<Player>>,
) {
    let aim_type = AimType::HomingClosest;

    match aim_type {
        AimType::Random => {
            let mut rng = rand::thread_rng();
            let new_velocity = random_velocity(&mut rng);
            for (_, mut projectile_velocity, _, _) in &mut projectile_query {
                if projectile_velocity.x == 0. && projectile_velocity.y == 0. {
                    projectile_velocity.x = new_velocity.x * PROJECTILESPEED;
                    projectile_velocity.y = new_velocity.y * PROJECTILESPEED;
                }
            }
        }
        AimType::Mouse => unimplemented!(),
        AimType::HomingMouse => unimplemented!(),
        AimType::Closest => {
            for (_, mut projectile_velocity, mut projectile_rotation, mut projectile_closest) in
                &mut projectile_query
            {
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
                    projectile_velocity.x = to_closest.x * PROJECTILESPEED;
                    projectile_velocity.y = to_closest.y * PROJECTILESPEED;
                }
            }
        }
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

                // get the vector from the projectile to the closest infected.
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
                projectile_velocity.x = to_closest.x * PROJECTILESPEED;
                projectile_velocity.y = to_closest.y * PROJECTILESPEED;
            }
        }
    }
}

pub fn update_projectile_lifetime(
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut ProjectileTimer)>,
    mut commands: Commands,
) {
    for (projectile_entity, mut projectile_timer) in projectile_query.iter_mut() {
        projectile_timer.timer.tick(time.delta());
        if projectile_timer.timer.just_finished() {
            commands.entity(projectile_entity).insert(Dead);
        }
    }
}

pub fn collide_projectile(
    mut commands: Commands,
    mut infected_query: Query<(Entity, &Transform, &mut Stats), With<Infected>>,
    mut projectile_transform_query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (infected_entity, infected_transform, mut infected_stats) in &mut infected_query {
        let infected_translation = infected_transform.translation;
        for (projectile_entity, projectile_transform) in &mut projectile_transform_query {
            let projectile_translation = projectile_transform.translation;
            let distance = Vec3::distance(projectile_translation, infected_translation);

            /*println!(
                "projectile : {}   infected  : {}     distance : {}",
                projectile_translation, infected_translation, distance,
            );*/

            if distance < PERSONSIZE {
                commands.entity(projectile_entity).insert(Dead);
                infected_stats.hit_points -= 1;
                if infected_stats.hit_points <= 0 {
                    commands.entity(infected_entity).insert(Dead);
                }
            }
        }
    }
}
