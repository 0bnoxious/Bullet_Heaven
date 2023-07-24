use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

pub const INFECTED_HP: i32 = 3;
pub const INFECTION_ODDS: i32 = 1; // 1 in x chance to infect
pub const INFECTED_COLOR: Color = Color::RED;
pub const INFECTED_SPEED: f32 = 50.;

use super::*;

#[derive(Component, Debug)]
pub struct Infected;

#[derive(Component, Debug)]
pub struct Target {
    pub target: Vec2,
}

#[derive(Bundle)]
pub struct InfectedBundle {
    infected: Infected,
    sprite_bundle: SpriteBundle,
    stats: Stats,
    layer: CollisionLayers,
    target: Target,
}

impl Default for InfectedBundle {
    fn default() -> Self {
        let square_sprite = Sprite {
            color: INFECTED_COLOR,
            custom_size: Some(Vec2 {
                x: DEFAULT_MOB_SIZE,
                y: DEFAULT_MOB_SIZE,
            }),
            ..default()
        };

        let mut rng = rand::thread_rng();
        let posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);

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
            },
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn infect(
    mut commands: Commands,
    mut is_healthy: Query<&mut InfectionAttemptTimer, With<Person>>,
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

        println!("contact between healthy  and infected");
        //attempt to infect once every INFECTION_ATTEMPT_DELAY_MS milliseconds
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            println!("infection attempt ");
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
    mut commands: Commands,
    player_quary: Query<&Position, With<Player>>,
    mut infected_querry: Query<&mut Target, With<Infected>>,
) {
    let player_position: Vec2 = player_quary.single().0;

    for mut infected_target in infected_querry.iter_mut() {
        infected_target.target = player_position;
    }
}

pub fn move_to_target(
    mut infected_query: Query<(&mut LinearVelocity, &Position, &Target), With<Infected>>,
) {
    for (mut velocity, position, target) in &mut infected_query {
        // get the vector from the infected to the target and normalise it.
        let to_player = (target.target - position.0).normalize();

        velocity.x = to_player.x * INFECTED_SPEED;
        velocity.y = to_player.y * INFECTED_SPEED;
    }
}
