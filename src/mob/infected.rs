use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

pub const INFECTED_HP: i32 = 3;
pub const DEFAULT_MOB_DEFENSE: i32 = 0;
pub const DEFAULT_MOB_DAMAGE: i32 = 1;
pub const DEFAULT_MOB_ATTACK_SPEED: f32 = 1.;
pub const DEFAULT_MOB_MOVEMENT_SPEED: f32 = 10.;
pub const INFECTED_MOVEMENT_SPEED: f32 = 50.;

pub const INFECTED_SIZE: f32 = 100.;
pub const INFECTED_COLOR: Color = Color::RED;
pub const INFECTION_ODDS: i32 = 1; // 1 in x chance to infect

use super::*;

#[derive(Component, Debug)]
pub struct Infected;

#[derive(Bundle)]
pub struct InfectedBundle {
    infected: Infected,
    sprite_bundle: SpriteBundle,
    stats: Stats,
    layer: CollisionLayers,
    target: Target,
    damage: Damage,
}

impl Default for InfectedBundle {
    fn default() -> Self {
        let square_sprite = Sprite {
            color: INFECTED_COLOR,
            custom_size: Some(Vec2 {
                x: INFECTED_SIZE,
                y: INFECTED_SIZE,
            }),
            ..default()
        };

        let mut rng = rand::thread_rng();
        let posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let dmg_vec: Vec<i32> = Vec::new();

        /*let detection_array = RayCaster {
            enabled: true,
            origin: Vec2 { x: posx, y: posy },
            direction: Vec2::X,
            ..Default::default()
        },*/

        Self {
            infected: Infected,
            target: Target {
                target: Vec2 { x: 0., y: 0. },
            },
            sprite_bundle: SpriteBundle {
                sprite: square_sprite,
                transform: Transform::from_translation(Vec3::new(posx, posy, 0.)),
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
            stats: Stats {
                hit_points: INFECTED_HP,
                movement_speed: INFECTED_MOVEMENT_SPEED,
                attack_speed: DEFAULT_MOB_ATTACK_SPEED,
                defense: DEFAULT_MOB_DEFENSE,
                damage: DEFAULT_MOB_DAMAGE,
            },
            damage: Damage { instances: dmg_vec },
            //detection_array: todo!(),
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn infect(
    mut commands: Commands,
    mut is_healthy: Query<&mut InfectionAttemptTimer, With<Healthy>>,
    is_sensor: Query<&Parent, With<Sensor>>,
    is_infected: Query<&Infected>,
    mut events: EventReader<Collision>,
    time: Res<Time>,
) {
    let mut collide = |entity_a: &Entity, entity_b: &Entity| -> bool {
        let Ok(parent) = is_sensor.get(*entity_a) else {
            return false;
        };

        if is_healthy.get(parent.get()).is_err() {
            return false;
        }

        if is_infected.get(*entity_b).is_err() {
            return false;
        }

        // get the healthy person's infection timer
        let Ok(mut timer) = is_healthy.get_mut(parent.get()) else {
            return false;
        };

        //attempt to infect once every INFECTION_ATTEMPT_DELAY_MS milliseconds
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            let mut rng = rand::thread_rng();
            // 1/INFECTION_ODDS chance to infect
            if rng.gen_range(0..INFECTION_ODDS) == 0 {
                commands
                    .entity(parent.get())
                    .insert(InfectedBundle::default());
            }
            return true;
        }

        false
    };

    // if entity is not a healthy person, flip'em.
    for Collision(contact) in events.iter() {
        if !collide(&contact.entity1, &contact.entity2) {
            collide(&contact.entity2, &contact.entity1);
        }
    }
}

pub fn target_player(
    player_quary: Query<&Position, With<Player>>,
    mut infected_querry: Query<&mut Target, With<Infected>>,
) {
    let player_position: Vec2 = player_quary.single().0;

    for mut infected_target in infected_querry.iter_mut() {
        infected_target.target = player_position;
    }
}

pub fn look_for_target() {}

pub fn move_to_target(
    mut infected_query: Query<(&mut LinearVelocity, &Position, &Target), With<Infected>>,
) {
    for (mut velocity, position, target) in &mut infected_query {
        let distance = Vec2::distance(position.0, target.target);
        //println!("distance between mob and player : {distance:?}");
        if Vec2::distance(position.0, target.target) > INFECTED_SIZE {
            // get the vector from the infected to the target and normalise it.
            let to_player = (target.target - position.0).normalize();

            velocity.x = to_player.x * INFECTED_MOVEMENT_SPEED;
            velocity.y = to_player.y * INFECTED_MOVEMENT_SPEED;
        } else {
            velocity.x = 0.;
            velocity.y = 0.;
        }
    }
}
