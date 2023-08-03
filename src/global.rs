use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::{mob::infected::DEFAULT_INFECTED_HP, projectile::Damage};

#[derive(Component, Debug)]
pub struct Stats {
    pub hit_points: i32,
    pub movement_speed: f32,
    pub attack_speed: f32,
    pub defense: i32,
    pub damage: i32,
}

pub const DEFAULT_HP: i32 = 1;
pub const DEFAULT_DEFENSE: i32 = 0;
pub const DEFAULT_DAMAGE: i32 = 0;
pub const DEFAULT_ATTACK_SPEED: f32 = 0.;
pub const DEFAULT_MOVEMENT_SPEED: f32 = 10.;

impl Default for Stats {
    fn default() -> Self {
        Self {
            hit_points: DEFAULT_HP,
            movement_speed: DEFAULT_MOVEMENT_SPEED,
            attack_speed: DEFAULT_ATTACK_SPEED,
            defense: DEFAULT_DEFENSE,
            damage: DEFAULT_DAMAGE,
        }
    }
}

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
    InfectedRanged,
    InfectedArmored,
    InfectedElite,
    InfectedCommander,
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
