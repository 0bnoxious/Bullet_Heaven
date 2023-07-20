use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::global::*;

use super::{person::*, Stats, INFECTEDCOUNT, INFECTEDHP, PERSONSIZE};

#[derive(Component)]
pub struct Infected;

pub fn spawn_infected(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let mut v = vec![];
    for _ in 0..INFECTEDCOUNT {
        let posx = rng.gen_range(-BOXSIZE..=BOXSIZE);
        let posy = rng.gen_range(-BOXSIZE..=BOXSIZE);

        v.push((
            Person {
                direction: generate_velocity(&mut rng),
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: (Some(Vec2 {
                        x: PERSONSIZE,
                        y: PERSONSIZE,
                    })),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(posx, posy, 0.)),
                ..default()
            },
            InfectTimer {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            },
            Infected,
            Stats {
                hit_points: INFECTEDHP,
            },
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
            if distance < PERSONSIZE {
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
