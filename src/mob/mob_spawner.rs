use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{global::Layer, map::BOXSIZE};

use super::{InfectTimer, Person, PERSONCOUNT, PERSONSIZE, PERSONSPEED};

pub fn spawn_person(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let mut v = vec![];
    for _ in 0..PERSONCOUNT {
        let posx = rng.gen_range(-BOXSIZE..=BOXSIZE);
        let posy = rng.gen_range(-BOXSIZE..=BOXSIZE);

        v.push((
            Person,
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
            RigidBody::Dynamic,
            Position(Vec2::new(posx, posy)),
            LinearVelocity(Vec2 {
                x: PERSONSPEED,
                y: PERSONSPEED,
            }),
            Collider::cuboid(PERSONSIZE, PERSONSIZE),
            CollisionLayers::new([Layer::Person], [Layer::Player]),
            LockedAxes::ROTATION_LOCKED,
            InfectTimer {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            },
        ));
    }
    commands.spawn_batch(v);
}
