use std::time::Duration;

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_xpbd_2d::{parry::shape::TypedShape, prelude::*};

use crate::{
    global::*,
    mob::{infected::Infected, Person, Stats},
    player::Player,
};

use super::{
    Projectile, ProjectileTimer, PROJECTILE_DAMAGE, PROJECTILE_LIFE_SPAN, PROJECTILE_SIZE,
    PROJECTILE_SPEED,
};

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
                        x: PROJECTILE_SIZE,
                        y: PROJECTILE_SIZE,
                    })),
                    ..default()
                },
                transform: Transform::from_translation(Vec3 {
                    x: PROJECTILE_SIZE,
                    y: PROJECTILE_SIZE,
                    z: 0.0,
                }),
                ..default()
            },
            RigidBody::Kinematic,
            Position(Vec2::new(player_position.x, player_position.y)),
            Collider::cuboid(PROJECTILE_SIZE * 2., PROJECTILE_SIZE * 2.),
            CollisionLayers::new([Layer::Projectile], [Layer::Person]),
            Closest {
                vec3: Vec3::new(0., 0., 0.),
            },
            ProjectileTimer {
                timer: Timer::new(Duration::from_secs(PROJECTILE_LIFE_SPAN), TimerMode::Once),
            },
            Mass::ZERO, // Sensor,
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
                    projectile_velocity.x = new_velocity.x * PROJECTILE_SPEED;
                    projectile_velocity.y = new_velocity.y * PROJECTILE_SPEED;
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
                    projectile_velocity.x = to_closest.x * PROJECTILE_SPEED;
                    projectile_velocity.y = to_closest.y * PROJECTILE_SPEED;
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

                // get the vector from the projectile to the closest infected and normaliwede it.
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

pub fn handle_projectile_collision(
    mut commands: Commands,
    mut infected_query: Query<&mut Stats, With<Infected>>,
    mut events: EventReader<CollisionStarted>,
    is_projectile: Query<&Projectile>,
) {
    let mut collide = |entity_a: &Entity, entity_b: &Entity| -> bool {
        if is_projectile.get(*entity_a).is_ok() {
            if let Ok(mut stats) = infected_query.get_mut(*entity_b) {
                stats.hit_points -= PROJECTILE_DAMAGE;
                if stats.hit_points <= 0 {
                    commands.entity(*entity_b).insert(Dead);
                }
                commands.entity(*entity_a).insert(Dead);
                return true;
            }
        }
        false
    };

    for CollisionStarted(entity_a, entity_b) in events.iter() {
        if !collide(entity_a, entity_b) {
            collide(entity_b, entity_a);
        }
    }
}
