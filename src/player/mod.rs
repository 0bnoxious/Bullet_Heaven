use std::time::Duration;

use bevy::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;

use crate::{
    global::*,
    weapon::{rifle::Rifle, Weapon, WeaponCoolDown},
};

use self::{
    action::move_player,
    input::{player_walks, PlayerAction, PlayerWalk},
    spawner::spawn_player,
};

pub mod action;
pub mod input;
pub mod spawner;

pub const DEFAULT_PLAYER_SIZE: u32 = 10;
pub const DEFAULT_PLAYER_HIT_POINTS: i32 = 999999999;
pub const DEFAULT_PLAYER_DEFENSE: i32 = 1;
pub const DEFAULT_PLAYER_ATTACK_SPEED: u32 = 1000;
pub const DEFAULT_PLAYER_MOVEMENT_SPEED: u32 = 3;
pub const DEFAULT_PLAYER_ANTI_MOB_SPAWN_SIZE: u32 = 200;
pub const DEFAULT_PLAYER_INVULNERABILITY: u32 = 1;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, player_walks))
            .add_event::<PlayerWalk>();
    }
}

#[derive(Component)]
pub struct Player;

pub fn default_player_stats() -> Stats {
    Stats {
        hit_points: DEFAULT_PLAYER_HIT_POINTS,
        movement_speed: DEFAULT_PLAYER_MOVEMENT_SPEED,
        attack_speed: DEFAULT_PLAYER_ATTACK_SPEED,
        defense: DEFAULT_PLAYER_DEFENSE,
        damage: 1,
    }
}
