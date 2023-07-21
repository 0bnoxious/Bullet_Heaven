use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::global::*;
use crate::map::*;
use crate::projectile::projectile_spawner::Projectile;

pub mod infected;
pub mod mob_spawner;

pub const PERSONCOUNT: i32 = 10;
pub const PERSONSPEED: f32 = 100.;
pub const PERSONSIZE: f32 = 10.;
pub const INFECTEDCOUNT: i32 = 30;
pub const INFECTEDHP: i32 = 1;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
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
    mut velocity_query: Query<&mut LinearVelocity, Without<Projectile>>,
    time: Res<Time>,
    mut timer_res: ResMut<PersonDirectionTimer>,
) {
    timer_res.timer.tick(time.delta());

    let mut rng = rand::thread_rng();
    for mut velocity in &mut velocity_query {
        if timer_res.timer.just_finished() {
            let new_velocity = random_velocity(&mut rng);
            velocity.x = new_velocity.x * PERSONSPEED;
            velocity.y = new_velocity.y * PERSONSPEED;
        }
    }
}
