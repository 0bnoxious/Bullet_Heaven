use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    global::{random_velocity, Layer},
    map::BOX_SIZE,
};

use super::{
    InfectTimer, Person, INFECTION_ATTEMPT_DELAY, PERSON_COUNT, PERSON_SIZE, PERSON_SPEED,
};

pub fn spawn_person(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let mut v = vec![];
    for _ in 0..PERSON_COUNT {
        let posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);

        v.push((
            Person,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: (Some(Vec2 {
                        x: PERSON_SIZE,
                        y: PERSON_SIZE,
                    })),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(posx, posy, 0.)),
                ..default()
            },
            RigidBody::Dynamic,
            Position(Vec2::new(posx, posy)),
            LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED),
            Collider::cuboid(PERSON_SIZE, PERSON_SIZE),
            CollisionLayers::new([Layer::Person], [Layer::Person]),
            LockedAxes::ROTATION_LOCKED,
            InfectTimer {
                timer: Timer::new(
                    Duration::from_millis(INFECTION_ATTEMPT_DELAY),
                    TimerMode::Repeating,
                ),
            },
        ));
    }
    commands.spawn_batch(v);
}
