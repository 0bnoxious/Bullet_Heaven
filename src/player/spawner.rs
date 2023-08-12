use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::global::*;

use super::{
    input::PlayerAction, AttackTimer, Player, DEFAULT_PLAYER_ATTACK_SPEED, DEFAULT_PLAYER_SIZE,
};

// must be added to the player entity
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    input_manager: InputManagerBundle<PlayerAction>,
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayerBundle {
            player: Player,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::player_input_map(),
                ..default()
            },
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: (Some(Vec2 {
                    x: DEFAULT_PLAYER_SIZE as f32,
                    y: DEFAULT_PLAYER_SIZE as f32,
                })),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Stats::default(),
        RigidBody::Kinematic,
        Position(Vec2::new(0., 0.)),
        Collider::cuboid(DEFAULT_PLAYER_SIZE as f32, DEFAULT_PLAYER_SIZE as f32),
        CollisionLayers::new([Layer::Player], [Layer::Person]),
        AttackTimer {
            timer: Timer::new(
                Duration::from_millis(DEFAULT_PLAYER_ATTACK_SPEED as u64),
                TimerMode::Repeating,
            ),
        },
        Name::new("Player"),
    ));
}
