use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::{mob::Stats, projectile::Damage};

#[derive(Component, Debug)]
pub enum AimType {
    Random,
    Closest,
    HomingClosest,
    //Mouse,
    //HomingMouse,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum MobType {
    Infected,
    InfectedButDifferent,
}

impl AimType {
    pub fn next(&self) -> Self {
        use AimType::*;
        match *self {
            Random => Closest,
            Closest => HomingClosest,
            HomingClosest => Random,
        }
    }
}

#[derive(PhysicsLayer)]
pub enum Layer {
    Default,
    Player,
    Person,
    PersonSensor,
    Infected,
    Projectile,
    Wall,
}

pub fn random_velocity(rng: &mut ThreadRng) -> Vec3 {
    let velx = rng.gen_range(-1.0..1.0);
    let vely = rng.gen_range(-1.0..1.0);

    Vec3::new(velx, vely, 0.)
}

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Closest {
    pub vec3: Vec3,
}

pub fn apply_damage(
    mut commands: Commands,
    mut damage_query: Query<(Entity, &mut Damage, &mut Stats)>,
) {
    for (entity, mut damage, mut stats) in &mut damage_query {
        let dmg_sum: i32 = damage.instances.iter().sum();
        stats.hit_points -= dmg_sum;
        damage.instances.clear();

        if stats.hit_points <= 0 {
            commands.entity(entity).insert(Dead);
        }
    }
}

pub fn despawn_dead(mut query: Query<Entity, With<Dead>>, mut commands: Commands) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
