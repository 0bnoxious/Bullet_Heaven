use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    global::{random_velocity, Layer},
    map::BOX_SIZE,
};

use super::*;

pub const INFECTED_COUNT: i32 = 40;

pub fn spawn_person(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let mut v = vec![];
    for _ in 0..PERSON_COUNT {
        v.push((
            Person,
            MobBundle {
                ..Default::default()
            },
            LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED),
            CollisionLayers::new([Layer::Person], [Layer::Person]),
            LockedAxes::ROTATION_LOCKED,
            InfectionAttemptTimer {
                timer: Timer::new(
                    Duration::from_millis(INFECTION_ATTEMPT_DELAY_MS),
                    TimerMode::Repeating,
                ),
            },
        ));
    }
    commands.spawn_batch(v);
}

pub fn spawn_infected(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let square_sprite = Sprite {
        color: Color::RED,
        custom_size: Some(Vec2 {
            x: DEFAULT_MOB_SIZE,
            y: DEFAULT_MOB_SIZE,
        }),
        ..default()
    };

    let mut v = vec![];
    for _ in 0..INFECTED_COUNT {
        let posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);

        v.push((
            Person,
            /*SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_translation(Vec3::new(posx, posy, 0.)),
                ..default()
            },*/
            RigidBody::Dynamic,
            Position(Vec2::new(posx, posy)),
            LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED),
            Collider::cuboid(DEFAULT_MOB_SIZE, DEFAULT_MOB_SIZE),
            LockedAxes::ROTATION_LOCKED,
            InfectedBundle::default(),
        ));
    }
    commands.spawn_batch(v);
}
