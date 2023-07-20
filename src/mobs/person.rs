use bevy::prelude::*;
use rand::Rng;
use std::time::Duration;

use crate::global::*;

use super::{PERSONCOUNT, PERSONSIZE, PERSONSPEED};

#[derive(Component)]
pub struct Person {
    pub direction: Vec3,
}

#[derive(Resource)]
pub struct PersonDirectionTimer {
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

pub fn update_person_direction(
    mut query: Query<&mut Person>,
    time: Res<Time>,
    mut timer_res: ResMut<PersonDirectionTimer>,
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
