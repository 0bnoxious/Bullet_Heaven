use bevy::prelude::*;
use bevy_xpbd_2d::prelude::{Collider, LockedAxes, Position, RigidBody};
use rand::Rng;
use std::time::Duration;

use super::*;

#[derive(Component, Debug)]
pub struct Infected;

pub fn spawn_infected(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let square_sprite = Sprite {
        color: Color::rgb(1., 0., 0.),
        custom_size: Some(Vec2 {
            x: PERSON_SIZE,
            y: PERSON_SIZE,
        }),
        ..default()
    };

    let mut v = vec![];
    for _ in 0..INFECTED_COUNT {
        let posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);

        v.push((
            Person,
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_translation(Vec3::new(posx, posy, 0.)),
                ..default()
            },
            RigidBody::Dynamic,
            Position(Vec2::new(posx, posy)),
            Collider::cuboid(PERSON_SIZE, PERSON_SIZE),
            LockedAxes::ROTATION_LOCKED,
            Stats {
                hit_points: INFECTED_HP,
            },
            InfectTimer {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            },
            Infected,
            Mass(50000.),
        ));
    }
    commands.spawn_batch(v);
}

#[allow(clippy::type_complexity)]
pub fn infect(
    mut commands: Commands,
    query_infected: Query<&Transform, With<Infected>>,
    mut query_healthy: Query<
        (Entity, &Transform, &mut Sprite, &mut InfectTimer),
        (With<Person>, Without<Infected>),
    >,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for infected_transform in &query_infected {
        for (entity, healthy_transform, mut sprite, mut infect_timer) in &mut query_healthy {
            let distance = infected_transform
                .translation
                .distance(healthy_transform.translation);
            if distance < PERSON_SIZE {
                //attempt to infect once every 1/5 second
                infect_timer.timer.tick(time.delta());
                if infect_timer.timer.finished() {
                    // 1/5 chance to infect
                    let infect = rng.gen_range(0..=4);
                    if infect == 4 {
                        sprite.color = Color::RED;
                        commands.entity(entity).insert(Infected);
                        commands.entity(entity).insert(Stats { hit_points: 3 });
                    }
                }
            }
        }
    }
}
