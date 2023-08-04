use std::time::Duration;

use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_xpbd_2d::prelude::*;

use super::{Projectile, ProjectileTimer, PROJECTILE_LIFE_SPAN, PROJECTILE_SIZE};
use crate::{global::*, targeting::HasTarget};

#[derive(SystemParam)]
pub struct ProjectileSpawner<'w, 's> {
    commands: Commands<'w, 's>,
}

impl<'w, 's> ProjectileSpawner<'w, 's> {
    pub fn spawn_projectile(&mut self, origin: Vec2, direction: Vec2, aim_type: AimType) {
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
            RigidBody::Dynamic,
            Position(origin),
            HasTarget {
                target_position: direction,
            },
            Collider::cuboid(PROJECTILE_SIZE * 2., PROJECTILE_SIZE * 2.),
            CollisionLayers::new([Layer::Projectile], [Layer::Infected]),
            ProjectileTimer {
                timer: Timer::new(Duration::from_secs(PROJECTILE_LIFE_SPAN), TimerMode::Once),
            },
            aim_type,
        ));
    }

    pub fn spawn_shotgun_projectile(&mut self, origin: Vec2, direction: Vec2, aim_type: AimType) {
        self.commands.spawn((
            Projectile,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::PURPLE,
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
            RigidBody::Dynamic,
            Position(origin),
            HasTarget {
                target_position: direction,
            },
            Collider::cuboid(PROJECTILE_SIZE * 2., PROJECTILE_SIZE * 2.),
            CollisionLayers::new([Layer::Projectile], [Layer::Infected]),
            ProjectileTimer {
                timer: Timer::new(Duration::from_secs(PROJECTILE_LIFE_SPAN), TimerMode::Once),
            },
            aim_type,
            Name::new("Shotgun bullet"),
        ));
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
