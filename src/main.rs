use bevy::sprite::SpriteBundle;
use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

pub const PERSONCOUNT: i32 = 50;
pub const PERSONSPEED: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_startup_system(populate)
        .add_system(move_population)
        .run()
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Person { is_infected: false });

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
}

pub fn populate(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut n = 0;

    while n < PERSONCOUNT {
        // Generate random number in the range [0, 99]
        let numx = rand::thread_rng().gen_range(0..200);
        let numy = rand::thread_rng().gen_range(0..200);

        // Circle
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: (Some(Vec2 { x: 10., y: 10. })),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(numx as f32, numy as f32, 0.)),
                ..default()
            },
            Person { is_infected: false },
        ));

        n += 1;
    }
}

fn move_population(
    mut query: Query<
        // components returned
        &mut Transform,
        // filter
        With<Person>,
    >,
    time: Res<Time>,
    mut timer_res: ResMut<TimerRes>, //metton jveu faire pause?
) {
    timer_res.timer.tick(time.delta());

    for mut transform in &mut query {
        if timer_res.timer.just_finished() {
            let mut direction = Vec3::new(0., 0., 0.);
            let numx = rand::thread_rng().gen_range(-1..=1);
            let numy = rand::thread_rng().gen_range(-1..=1);
            println!("numx : {}   numy : {}", numx.to_string(), numy.to_string());
            direction += Vec3::new(numx as f32, numy as f32, 0.);

            transform.translation += direction * PERSONSPEED * time.delta_seconds();
        }
    }
}

pub fn infect(query: Query<&Person>) {
    for person in &query {
        println!("Is infected? : {}", person.is_infected);
    }
}
