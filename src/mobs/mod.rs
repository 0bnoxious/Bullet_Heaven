use bevy::prelude::*;

pub mod infected;
pub mod person;

pub const PERSONCOUNT: i32 = 1;
pub const PERSONSPEED: f32 = 50.;
pub const PERSONSIZE: f32 = 10.;
pub const INFECTEDCOUNT: i32 = 1000;
pub const INFECTEDHP: i32 = 1;

#[derive(Component)]
pub struct Stats {
    pub hit_points: i32,
}
