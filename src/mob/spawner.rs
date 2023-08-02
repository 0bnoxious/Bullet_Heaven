use std::time::Duration;

use bevy::{prelude::*, reflect::erased_serde::__private::serde::__private::de};
use bevy_xpbd_2d::{math::Vector, prelude::*};
use rand::Rng;

use crate::{
    global::{random_velocity, Layer},
    map::BOX_SIZE,
    player::PLAYER_ANTI_MOB_SPAWN_SIZE,
};

use super::*;

pub const MAX_MOB_COUNT: i32 = 500;
pub const INFECTED_RATIO: i32 = 1;
pub const HEALTHY_RATIO: i32 = 1;
pub const INFECTED_COUNT: i32 = (MAX_MOB_COUNT / (INFECTED_RATIO + HEALTHY_RATIO)) * INFECTED_RATIO;
pub const HEALTHY_COUNT: i32 = (MAX_MOB_COUNT / (INFECTED_RATIO + HEALTHY_RATIO)) * HEALTHY_RATIO;

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

#[derive(SystemParam)]
pub struct MobSpawner<'w, 's> {
    commands: Commands<'w, 's>,
    player_pos_query: Query<'w, 's, &'static Position, With<Player>>,
}

impl<'w, 's> MobSpawner<'w, 's> {
    pub fn spawn_mob(&mut self, mob_type: MobType, mob_count: u64) {
        println!("type of mob being spawned : {mob_type:?}");

        match mob_type {
            MobType::Infected => {
                for _ in 0..mob_count {
                    let mob_pos = safe_spawn_location(self.player_pos_query.single());

                    self.commands.spawn((
                        RigidBody::Dynamic,
                        Position(mob_pos),
                        //LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED), // <--- applying random velocity sometimes break the physics engine
                        LinearVelocity(Vec2::new(0., 0.)),
                        Collider::cuboid(DEFAULT_MOB_SIZE, DEFAULT_MOB_SIZE),
                        LockedAxes::ROTATION_LOCKED,
                        InfectedBundle::default(),
                        Name::new("Infected"),
                    ));
                }
            }
            MobType::InfectedRanged => {
                let square_sprite = Sprite {
                    color: Color::ALICE_BLUE,
                    custom_size: Some(Vec2 {
                        x: DEFAULT_MOB_SIZE,
                        y: DEFAULT_MOB_SIZE,
                    }),
                    ..default()
                };
                for _ in 0..mob_count {
                    let mob_pos = safe_spawn_location(self.player_pos_query.single());

                    self.commands.spawn((
                        RigidBody::Dynamic,
                        Position(mob_pos),
                        //LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED), // <--- applying random velocity sometimes break the physics engine
                        LinearVelocity(Vec2::new(0., 0.)),
                        Collider::cuboid(DEFAULT_MOB_SIZE, DEFAULT_MOB_SIZE),
                        LockedAxes::ROTATION_LOCKED,
                        InfectedBundle {
                            infected: Infected,
                            sprite_bundle: SpriteBundle {
                                sprite: square_sprite.clone(),
                                ..default()
                            },
                            ..default()
                        },
                        Name::new("InfectedRanged"),
                    ));
                }
            }
            MobType::InfectedArmored => todo!(),
            MobType::InfectedElite => todo!(),
            MobType::InfectedCommander => todo!(),
        }
    }
}

pub fn spawn_healthy(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    for _ in 0..HEALTHY_COUNT {
        commands
            .spawn((
                Healthy,
                MobBundle::default(),
                LinearVelocity(random_velocity(&mut rng).truncate() * HEALTHY_MOVEMENT_SPEED),
                CollisionLayers::new([Layer::Person], [Layer::Person]),
                LockedAxes::ROTATION_LOCKED,
                InfectionAttemptTimer {
                    timer: Timer::new(
                        Duration::from_millis(INFECTION_ATTEMPT_DELAY_MS),
                        TimerMode::Repeating,
                    ),
                },
            ))
            .with_children(|c| {
                c.spawn((
                    RigidBody::Kinematic,
                    Collider::cuboid(DEFAULT_MOB_SIZE, DEFAULT_MOB_SIZE),
                    Sensor,
                    CollisionLayers::new([Layer::PersonSensor], [Layer::Infected]),
                    SpatialBundle::default(),
                ));
            });
    }
}

pub fn spawn_infected(
    mut commands: Commands,
    infected_querry: Query<&Infected>,
    mut spawn_timer_res: ResMut<SpawnTimer>,
    player_pos_query: Query<&Position, With<Player>>,
    time: Res<Time>,
) {
    spawn_timer_res.timer.tick(time.delta());
    if spawn_timer_res.timer.just_finished() {
        let missing_infected_count = MAX_MOB_COUNT - infected_querry.iter().count() as i32;

        for _ in 0..missing_infected_count {
            let mob_pos = safe_spawn_location(player_pos_query.single());

            commands.spawn((
                RigidBody::Dynamic,
                Position(mob_pos),
                //LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED), // <--- applying random velocity sometimes break the physics engine
                LinearVelocity(Vec2::new(0., 0.)),
                Collider::cuboid(DEFAULT_MOB_SIZE, DEFAULT_MOB_SIZE),
                LockedAxes::ROTATION_LOCKED,
                InfectedBundle::default(),
            ));
        }
    }
}

pub fn safe_spawn_location(player_pos: &Position) -> Vec2 {
    let mut rng = rand::thread_rng();
    let mut mob_posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
    let mut mob_posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);

    // if the mob spawns within the player safe zone
    while Vec2::distance(
        Vec2 {
            x: mob_posx,
            y: mob_posy,
        },
        Vec2 {
            x: player_pos.x,
            y: player_pos.y,
        },
    ) < PLAYER_ANTI_MOB_SPAWN_SIZE
    {
        // try another spot
        println!("mob spawned within safe zone! relocating spawn location...");
        mob_posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        mob_posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
    }

    Vec2 {
        x: mob_posx,
        y: mob_posy,
    }
}

/*pub fn is_within_player_safe_zone(posx: f32, posy: f32, player_pos: &Position) -> bool {
    println!("mob spawned at location : ({},{})", posx, posy);
    println!("player location : {player_pos:?}");
    // if the mob spawn within the PLAYER_ANTI_MOB_SPAWN zone
    if posx < player_pos.x + PLAYER_ANTI_MOB_SPAWN_SIZE
        || posx > player_pos.x - PLAYER_ANTI_MOB_SPAWN_SIZE
            && posy < player_pos.y + PLAYER_ANTI_MOB_SPAWN_SIZE
        || posy > player_pos.y + PLAYER_ANTI_MOB_SPAWN_SIZE
    {
        println!("mob spawned within safe zone! relocating spawn location...");
        return true;
    }
    false
}*/
