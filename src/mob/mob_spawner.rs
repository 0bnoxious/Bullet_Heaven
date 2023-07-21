use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{global::Layer, map::BOX_SIZE};

use super::{InfectTimer, Person, PERSON_COUNT, PERSON_SIZE, PERSON_SPEED};

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
            LinearVelocity(Vec2 {
                x: PERSON_SPEED,
                y: PERSON_SPEED,
            }),
            Collider::cuboid(PERSON_SIZE, PERSON_SIZE),
            CollisionLayers::new([Layer::Person], [Layer::Player]),
            LockedAxes::ROTATION_LOCKED,
            InfectTimer {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            },
        ));
    }
    commands.spawn_batch(v);
}
