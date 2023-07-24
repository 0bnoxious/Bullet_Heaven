use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    global::{random_velocity, Layer},
    map::BOX_SIZE,
};

use super::*;

pub const INFECTED_COUNT: i32 = 1;
pub const PERSON_COUNT: i32 = 2;
pub const MAX_MOB_COUNT: i32 = 500;

pub fn spawn_person(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..PERSON_COUNT {
        commands
            .spawn((
                Person,
                MobBundle::default(),
                LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED),
                CollisionLayers::new([Layer::Person], [Layer::Person]),
                LockedAxes::ROTATION_LOCKED,
                InfectionAttemptTimer {
                    timer: Timer::new(
                        Duration::from_millis(INFECTION_ATTEMPT_DELAY_MS),
                        TimerMode::Repeating,
                    ),
                },
            ))
            .with_children(|c| {
                c.spawn((
                    RigidBody::Kinematic,
                    Collider::cuboid(DEFAULT_MOB_SIZE, DEFAULT_MOB_SIZE),
                    Sensor,
                    CollisionLayers::new([Layer::PersonSensor], [Layer::Infected]),
                    SpatialBundle::default(),
                ));
            });
    }
}

pub fn spawn_infected(mut commands: Commands, infected_querry: Query<&Infected>) {
    let mut infected_count = 0;
    for infected in infected_querry.iter() {
        infected_count += 1;
    }

    if infected_count >= MAX_MOB_COUNT {
        return;
    }

    let mut rng = rand::thread_rng();

    let mut v = vec![];
    for _ in 0..INFECTED_COUNT {
        let posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);

        v.push((
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
