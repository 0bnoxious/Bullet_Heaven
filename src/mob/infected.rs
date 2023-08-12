use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub const DEFAULT_INFECTED_HP: i32 = 3;
pub const DEFAULT_INFECTED_DEFENSE: i32 = 0;
pub const DEFAULT_INFECTED_DAMAGE: i32 = 1;
pub const DEFAULT_INFECTED_ATTACK_SPEED: u32 = 1;
pub const DEFAULT_INFECTED_MOVEMENT_SPEED: u32 = 200;

//pub const DEFAULT_INFECTION_ODDS: i32 = 1; // 1 in x chance to infect
pub const DEFAULT_INFECTED_COLOR: Color = Color::RED;
pub const DEFAULT_INFECTED_RANGED_COLOR: Color = Color::WHITE;
pub const DEFAULT_INFECTED_SIZE: f32 = 10.;

use super::*;

pub fn default_infected_stats() -> Stats {
    Stats {
        hit_points: DEFAULT_INFECTED_HP,
        movement_speed: DEFAULT_INFECTED_MOVEMENT_SPEED,
        attack_speed: DEFAULT_INFECTED_ATTACK_SPEED,
        defense: DEFAULT_INFECTED_DEFENSE,
        damage: DEFAULT_INFECTED_DAMAGE,
    }
}

#[derive(Component, Debug)]
pub struct Infected;

#[derive(Component, Debug)]
pub struct Ranged {}

#[derive(Bundle)]
pub struct InfectedBundle {
    pub infected: Infected,
    pub sprite_bundle: SpriteBundle,
    pub stats: Stats,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub layer: CollisionLayers,
    pub axes: LockedAxes,
    pub damage: Damage,
    pub name: Name,
}

#[derive(Bundle)]
pub struct InfectedRangedBundle {
    pub infected: Infected,
    pub sprite_bundle: SpriteBundle,
    pub stats: Stats,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub layer: CollisionLayers,
    pub axes: LockedAxes,
    pub damage: Damage,
    pub name: Name,
}

impl Default for InfectedBundle {
    fn default() -> Self {
        let square_sprite = Sprite {
            color: DEFAULT_INFECTED_COLOR,
            custom_size: Some(Vec2 {
                x: DEFAULT_INFECTED_SIZE,
                y: DEFAULT_INFECTED_SIZE,
            }),
            ..default()
        };

        let dmg_vec: Vec<i32> = Vec::new();

        Self {
            infected: Infected,
            sprite_bundle: SpriteBundle {
                sprite: square_sprite,
                ..default()
            },
            layer: CollisionLayers::new(
                [Layer::Infected],
                [
                    Layer::Player,
                    Layer::Projectile,
                    Layer::Infected,
                    Layer::PersonSensor,
                ],
            ),
            stats: default_infected_stats(),
            damage: Damage { instances: dmg_vec },
            collider: Collider::cuboid(DEFAULT_INFECTED_SIZE, DEFAULT_INFECTED_SIZE),
            rigid_body: RigidBody::Dynamic,
            axes: LockedAxes::ROTATION_LOCKED,
            name: Name::new("Infected"),
        }
    }
}

impl Default for InfectedRangedBundle {
    fn default() -> Self {
        let square_sprite = Sprite {
            color: DEFAULT_INFECTED_RANGED_COLOR,
            custom_size: Some(Vec2 {
                x: DEFAULT_INFECTED_SIZE,
                y: DEFAULT_INFECTED_SIZE,
            }),
            ..default()
        };

        let dmg_vec: Vec<i32> = Vec::new();

        Self {
            infected: Infected,
            sprite_bundle: SpriteBundle {
                sprite: square_sprite,
                ..default()
            },
            layer: CollisionLayers::new(
                [Layer::Infected],
                [
                    Layer::Player,
                    Layer::Projectile,
                    Layer::Infected,
                    Layer::PersonSensor,
                ],
            ),
            stats: default_infected_stats(),
            damage: Damage { instances: dmg_vec },
            collider: Collider::cuboid(DEFAULT_INFECTED_SIZE, DEFAULT_INFECTED_SIZE),
            rigid_body: RigidBody::Dynamic,
            axes: LockedAxes::ROTATION_LOCKED,
            name: Name::new("Ranged Infected"),
        }
    }
}

// #[allow(clippy::type_complexity)]
// pub fn infect(
//     mut commands: Commands,
//     mut is_healthy: Query<&mut InfectionAttemptTimer, With<Healthy>>,
//     is_sensor: Query<&Parent, With<Sensor>>,
//     is_infected: Query<&Infected>,
//     mut events: EventReader<Collision>,
//     time: Res<Time>,
// ) {
//     let mut collide = |entity_a: &Entity, entity_b: &Entity| -> bool {
//         let Ok(parent) = is_sensor.get(*entity_a) else {
//             return false;
//         };

//         if is_healthy.get(parent.get()).is_err() {
//             return false;
//         }

//         if is_infected.get(*entity_b).is_err() {
//             return false;
//         }

//         // get the healthy person's infection timer
//         let Ok(mut timer) = is_healthy.get_mut(parent.get()) else {
//             return false;
//         };

//         //attempt to infect once every INFECTION_ATTEMPT_DELAY_MS milliseconds
//         timer.timer.tick(time.delta());
//         if timer.timer.finished() {
//             let mut rng = rand::thread_rng();
//             // 1/INFECTION_ODDS chance to infect
//             if rng.gen_range(0..DEFAULT_INFECTION_ODDS) == 0 {
//                 commands
//                     .entity(parent.get())
//                     .insert(InfectedBundle::default());
//             }
//             return true;
//         }

//         false
//     };

//     // if entity is not a healthy person, flip'em.
//     for Collision(contact) in events.iter() {
//         if !collide(&contact.entity1, &contact.entity2) {
//             collide(&contact.entity2, &contact.entity1);
//         }
//     }
// }
