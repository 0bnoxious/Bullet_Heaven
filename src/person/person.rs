use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::{generate_velocity, Stats, BOXSIZE};

pub const PERSONCOUNT: i32 = 100;
pub const PERSONSPEED: f32 = 50.;
pub const PERSONSIZE: f32 = 10.;
pub const INFECTEDCOUNT: i32 = 1000;
pub const INFECTEDHP: i32 = 1;

#[derive(Component)]
pub struct Person {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Infected;

#[derive(Resource)]
pub struct PersonTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct InfectTimer {
    pub timer: Timer,
}

pub fn spawn_person(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let mut v = vec![];
    for _ in 0..PERSONCOUNT {
        let posx = rng.gen_range(-BOXSIZE..=BOXSIZE);
        let posy = rng.gen_range(-BOXSIZE..=BOXSIZE);

        v.push((
            Person {
                direction: generate_velocity(&mut rng),
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
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
        ));
    }
    commands.spawn_batch(v);
}

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

pub fn update_person_direction(
    mut query: Query<&mut Person>,
    time: Res<Time>,
    mut timer_res: ResMut<PersonTimer>,
) {
    timer_res.timer.tick(time.delta());

    let mut rng = rand::thread_rng();
    for mut person in &mut query {
        if timer_res.timer.just_finished() {
            person.direction = generate_velocity(&mut rng);
        }
    }
}

pub fn move_population(mut query: Query<(&mut Transform, &Person)>, time: Res<Time>) {
    for (mut transform, person) in &mut query {
        transform.translation += person.direction * PERSONSPEED * time.delta_seconds();
    }
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
