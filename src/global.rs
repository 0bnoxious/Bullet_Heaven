use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

pub const BOXSIZE: f32 = 720.;

#[derive(Resource)]
pub enum AimType {
    Random,
    Closest,
    HomingClosest,
    Mouse,
    HomingMouse,
    Direction,
}

pub fn generate_velocity(rng: &mut ThreadRng) -> Vec3 {
    let velx = rng.gen_range(-1.0..1.0);
    let vely = rng.gen_range(-1.0..1.0);

    Vec3::new(velx, vely, 0.)
}
