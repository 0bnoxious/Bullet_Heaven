use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{map::BOX_SIZE, player::PLAYER_ANTI_MOB_SPAWN_SIZE, targeting::Target};

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
        match mob_type {
            MobType::Infected => {
                for _ in 0..mob_count {
                    let mob_pos = safe_spawn_location(self.player_pos_query.single());

                    self.commands.spawn((
                        Mob,
                        Target,
                        InfectedBundle::default(),
                        Position(mob_pos),
                    ));
                }
            }

            MobType::InfectedRanged => {
                for _ in 0..mob_count {
                    let mob_pos = safe_spawn_location(self.player_pos_query.single());

                    self.commands.spawn((
                        Mob,
                        InfectedRangedBundle { ..default() },
                        Position(mob_pos),
                    ));
                }
            }

            MobType::InfectedArmored => todo!(),
            MobType::InfectedElite => todo!(),
            MobType::InfectedCommander => todo!(),
        }
    }
}

// to avoid unfair spawn position relative to the player
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
        mob_posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        mob_posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
    }

    Vec2 {
        x: mob_posx,
        y: mob_posy,
    }
}
