use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::global::*;
use crate::map::*;
use crate::player::Player;
use crate::projectile::Projectile;

pub mod infected;
pub mod mob_spawner;

pub const PERSON_COUNT: i32 = 1;
pub const PERSON_SPEED: f32 = 10.;
pub const PERSON_SIZE: f32 = 10.;
pub const INFECTED_COUNT: i32 = 1;
pub const INFECTED_HP: i32 = 6000;

#[derive(Component)]
pub struct Person;

#[derive(Component, Debug)]
pub struct Stats {
    pub hit_points: i32,
}

#[derive(Resource)]
pub struct PersonDirectionTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct InfectTimer {
    pub timer: Timer,
}

pub fn update_person_velocity(
    mut velocity_query: Query<&mut LinearVelocity, (Without<Projectile>, Without<Player>)>,
    time: Res<Time>,
    mut timer_res: ResMut<PersonDirectionTimer>,
) {
    timer_res.timer.tick(time.delta());

    let mut rng = rand::thread_rng();
    for mut velocity in &mut velocity_query {
        if timer_res.timer.just_finished() {
            let new_velocity = random_velocity(&mut rng);
            velocity.x = new_velocity.x * PERSON_SPEED;
            velocity.y = new_velocity.y * PERSON_SPEED;
        }
    }
}
