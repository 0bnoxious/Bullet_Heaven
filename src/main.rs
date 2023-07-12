use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::gamepad::GamepadButton;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use rand::Rng;
use std::time::Duration;

pub const PERSONCOUNT: i32 = 5000;
pub const PERSONSPEED: f32 = 50.;
pub const PERSONSIZE: f32 = 10.;
pub const BOXSIZE: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(populate)
        .add_system(move_population)
        .add_system(update_population_direction)
        .add_system(infect)
        .add_system(define_space)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
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
struct InfectTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Person {
    pub is_infected: bool,
    pub direction: Vec3,
}

pub fn populate(mut commands: Commands) {
    let mut n = 0;

    //patient 0
    commands.spawn((
        Person {
            is_infected: true,
            direction: generate_velocity(),
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
        InfectTimer {
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
        },
    ));

    while n < PERSONCOUNT {
        let posx = rand::thread_rng().gen_range(-100..=100);
        let posy = rand::thread_rng().gen_range(-100..=100);

        commands.spawn((
            Person {
                is_infected: false,
                direction: generate_velocity(),
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
                transform: Transform::from_translation(Vec3::new(posx as f32, posy as f32, 0.)),
                ..default()
            },
            InfectTimer {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
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
            person.direction = generate_velocity() * PERSONSPEED * time.delta_seconds();
        }
    }
}

fn infect(
    mut query: Query<(&mut Transform, &mut Person, &mut Sprite, &mut InfectTimer)>,
    time: Res<Time>,
) {
    let combinations = &mut query.iter_combinations_mut();
    while let Some(
        [(tranform1, mut person1, mut sprite1, mut infect_timer1), (transform2, mut person2, mut sprite2, mut infect_timer2)],
    ) = combinations.fetch_next()
    {
        let distance = tranform1.translation.distance(transform2.translation);

        if (person2.is_infected || person1.is_infected) && distance < PERSONSIZE {
            // attempt to infect once every 1/5 second
            infect_timer2.timer.tick(time.delta());
            if infect_timer2.timer.finished() {
                // 1/5 chance to infect
                let infect = rand::thread_rng().gen_range(0..=4);
                if infect == 4 {
                    person1.is_infected = true;
                    person2.is_infected = true;
                    sprite1.color = Color::RED;
                    sprite2.color = Color::RED;
                }
            }
        }
    }
}

fn define_space(mut query: Query<&mut Transform, With<Person>>) {
    let minxy = (-BOXSIZE / 2.) - PERSONSIZE / 2.;
    let maxxy = (BOXSIZE / 2.) - PERSONSIZE / 2.;

    for mut transform in query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < minxy {
            translation.x = minxy;
        } else if translation.x > maxxy {
            translation.x = maxxy
        }
        if translation.y < minxy {
            translation.y = minxy;
        } else if translation.y > maxxy {
            translation.y = maxxy
        }

        transform.translation = translation
    }
}

fn generate_velocity() -> Vec3 {
    let velx = rand::thread_rng().gen_range(-1.0..1.0);
    let vely = rand::thread_rng().gen_range(-1.0..1.0);

    Vec3::new(velx, vely, 0.)
}
//fn printoute(mut query: Query) {}
