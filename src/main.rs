use bevy::sprite::SpriteBundle;
use std::time::Duration;

use bevy::prelude::*;
use rand::{rngs::mock::StepRng, seq::IteratorRandom, thread_rng, Rng};

pub const PERSONCOUNT: i32 = 50;
pub const PERSONSPEED: f32 = 50.;
pub const PERSONSIZE: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(populate)
        .add_system(move_population)
        .add_system(update_population_direction)
        .add_system(infect)
        .add_system(change_color)
        .run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(TimerRes {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });
}

#[derive(Resource)]
struct TimerRes {
    timer: Timer,
}

#[derive(Component)]
pub struct Person {
    pub is_infected: bool,
    pub color: Color,
    pub direction: Vec3,
}

pub fn populate(mut commands: Commands) {
    let mut n = 0;

    //patient 0
    commands.spawn((
        Person {
            is_infected: true,
            color: Color::RED,
            direction: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
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
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
    ));

    while n < PERSONCOUNT {
        // Generate random number in the range [0, 99]
        let numx = rand::thread_rng().gen_range(-100..=100);
        let numy = rand::thread_rng().gen_range(-100..=100);

        commands.spawn((
            Person {
                is_infected: false,
                color: Color::GREEN,
                direction: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
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
                transform: Transform::from_translation(Vec3::new(numx as f32, numy as f32, 0.)),
                ..default()
            },
        ));

        n += 1;
    }
}

fn move_population(mut query: Query<(&mut Transform, &Person)>, time: Res<Time>) {
    for (mut transform, person) in &mut query.iter_mut() {
        transform.translation += person.direction * PERSONSPEED * time.delta_seconds();
    }
}

fn update_population_direction(
    mut query: Query<&mut Person>,
    time: Res<Time>,
    mut timer_res: ResMut<TimerRes>,
) {
    timer_res.timer.tick(time.delta());

    for mut person in &mut query {
        if timer_res.timer.just_finished() {
            let mut direction = Vec3::new(0., 0., 0.);

            let mut rng = thread_rng();
            let v = vec![-1, 1];
            let numx = v.iter().choose(&mut rng).unwrap();
            let numy = v.iter().choose(&mut rng).unwrap();

            //let numx = rand::thread_rng().gen_range(-1..=1);
            //let numy = rand::thread_rng().gen_range(-1..=1);
            direction += Vec3::new(*numx as f32, *numy as f32, 0.);

            person.direction = direction * PERSONSPEED * time.delta_seconds();
        }
    }
}

fn infect(mut query: Query<(&mut Transform, &mut Person)>) {
    let combinations = &mut query.iter_combinations_mut();
    while let Some([(tranform1, mut person1), (transform2, mut person2)]) =
        combinations.fetch_next()
    {
        let distance = tranform1.translation.distance(transform2.translation);
        if (person2.is_infected || person1.is_infected) && distance < PERSONSIZE {
            person1.is_infected = true;
            person2.is_infected = true;
        }
    }
}

fn change_color(mut query: Query<(&Person, &mut Sprite)>) {
    for (person, mut sprite) in &mut query {
        if person.is_infected {
            sprite.color = Color::RED;
        } else {
            sprite.color = Color::GREEN;
        }
    }
}
