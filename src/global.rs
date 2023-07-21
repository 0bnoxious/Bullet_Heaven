use bevy::prelude::*;
use rand::{rngs::ThreadRng, Rng};

#[derive(Resource)]
pub enum AimType {
    Random,
    Closest,
    HomingClosest,
    Mouse,
    HomingMouse,
}

pub fn random_velocity(rng: &mut ThreadRng) -> Vec3 {
    let velx = rng.gen_range(-1.0..1.0);
    let vely = rng.gen_range(-1.0..1.0);

    Vec3::new(velx, vely, 0.)
}

#[derive(Component)]
pub struct Dead;