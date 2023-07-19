use bevy::prelude::*;

use crate::{generate_velocity, person::person::*, player::player::*, Dead, Stats};

pub const PROJECTILESPEED: f32 = 200.;
pub const PROJECTILESIZE: f32 = 8.;
pub const PROJECTILELIFESPAN: u64 = 1;

#[derive(Component)]
pub struct ProjectileTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec3,
}

#[allow(clippy::type_complexity)]
pub fn move_projectile(
    mut projectile_query: Query<(&mut Transform, &mut Projectile)>,
    infected_query: Query<&Transform, (With<Infected>, With<Person>, Without<Projectile>)>,
    time: Res<Time>,
) {
    let aim_type = AimType::Random;

    match aim_type {
        AimType::Random => {
            let mut rng = rand::thread_rng();
            let velocity = generate_velocity(&mut rng);
            for (mut transform, mut projectile) in &mut projectile_query {
                if projectile.direction == Vec3::ZERO {
                    projectile.direction += velocity;
                    transform.translation +=
                        projectile.direction.normalize() * PROJECTILESPEED * time.delta_seconds()
                } else {
                    transform.translation +=
                        projectile.direction.normalize() * PROJECTILESPEED * time.delta_seconds();
                }
            }
        }
        AimType::HomingClosest => {
            let mut closest_distance = 1000.;
            let mut closest_infected_translation = Vec3::ZERO;

            for (mut projectile_transform, _) in &mut projectile_query {
                let projectile_translation = projectile_transform.translation;

                for infected_transform in &mut infected_query.iter() {
                    let infected_translation = infected_transform.translation;

                    let distance = Vec3::distance(projectile_translation, infected_translation);

                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_infected_translation = infected_translation;
                    }
                }

                // get the vector from the projectile to the closest infected.
                let to_closest = closest_infected_translation - projectile_translation;

                // get the quaternion to rotate from the initial projectile facing direction to the direction
                // facing the closest infected
                let rotate_to_infected = Quat::from_rotation_arc(Vec3::Y, to_closest);

                // rotate the projectile to face the closest infected
                projectile_transform.rotation = rotate_to_infected;
                projectile_transform.translation +=
                    to_closest.normalize() * PROJECTILESPEED * time.delta_seconds();
            }
        }
        AimType::Direction => unimplemented!(),
        AimType::Mouse => unimplemented!(),
        AimType::Closest => {
            for (mut projectile_transform, mut projectile) in &mut projectile_query {
                if projectile.direction == Vec3::ZERO {
                    let mut closest_distance = 1000.;
                    let mut closest_infected_translation = Vec3::ZERO;

                    let projectile_translation = projectile_transform.translation;

                    for infected_transform in &mut infected_query.iter() {
                        let infected_translation = infected_transform.translation;

                        let distance = Vec3::distance(projectile_translation, infected_translation);

                        if distance < closest_distance {
                            closest_distance = distance;
                            closest_infected_translation = infected_translation;
                        }
                    }

                    // get the vector from the projectile to the closest infected.
                    let to_closest = closest_infected_translation - projectile_translation;

                    // get the quaternion to rotate from the initial projectile facing direction to the direction
                    // facing the closest infected
                    let rotate_to_infected = Quat::from_rotation_arc(Vec3::Y, to_closest);

                    // rotate the projectile to face the closest infected
                    projectile_transform.rotation = rotate_to_infected;
                    projectile.direction +=
                        to_closest.normalize() * PROJECTILESPEED * time.delta_seconds();
                }

                projectile_transform.translation +=
                    projectile.direction * PROJECTILESPEED * time.delta_seconds();
            }
        }
        AimType::HomingMouse => unimplemented!(),
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
