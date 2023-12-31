use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::{global::*, mob::infected::Infected};

pub mod movement;
pub mod spawner;

pub const PROJECTILE_SIZE: f32 = 8.;
pub const PROJECTILE_SPEED: f32 = 500.;
pub const PROJECTILE_DAMAGE: i32 = 1;
pub const PROJECTILE_LIFE_SPAN: u64 = 1;

#[derive(Component, Debug)]
pub struct Projectile;

#[derive(Component, Debug)]
pub struct Homing;

#[derive(Component)]
pub struct Damage {
    pub instances: Vec<i32>,
}

#[derive(Component)]
pub struct ProjectileTimer {
    pub timer: Timer,
}

#[derive(Component, Debug)]
pub enum ProjectileType {
    Bullet,
    //Explosive,
    //Lazer,
    //Orbiting,
    //Lobbing,
}

pub fn handle_projectile_collision(
    mut commands: Commands,
    mut infected_query: Query<&mut Damage, With<Infected>>,
    mut events: EventReader<CollisionStarted>,
    is_projectile: Query<&Projectile>,
) {
    let mut collide = |entity_a: &Entity, entity_b: &Entity| -> bool {
        if is_projectile.get(*entity_a).is_ok() {
            // get the target's damage stack
            if let Ok(mut damage) = infected_query.get_mut(*entity_b) {
                // add the projectile damage to the damage stack
                damage.instances.push(PROJECTILE_DAMAGE);
                // delete projectile after contact
                commands.entity(*entity_a).insert(Dead);
                return true;
            }
        }
        false
    };

    // if entity a is not a projectile, flip'em.
    for CollisionStarted(entity_a, entity_b) in events.iter() {
        if !collide(entity_a, entity_b) {
            collide(entity_b, entity_a);
        }
    }
}
