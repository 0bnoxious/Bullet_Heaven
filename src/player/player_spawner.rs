use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::global::AimType;
use crate::global::*;
use crate::weapon::shotgun::Shotgun;

use super::{
    player_input::PlayerAction, AttackTimer, Player, ATTACK_SPEED, PLAYER_AIM_TYPE, PLAYER_SIZE,
};

// must be added to the player entity
#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    input_manager: InputManagerBundle<PlayerAction>,
    aim_type: AimType,
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        PlayerBundle {
            player: Player,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::player_input_map(),
                ..default()
            },
            aim_type: PLAYER_AIM_TYPE,
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: (Some(Vec2 {
                    x: PLAYER_SIZE,
                    y: PLAYER_SIZE,
                })),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        RigidBody::Kinematic,
        Position(Vec2::new(0., 0.)),
        Collider::cuboid(PLAYER_SIZE, PLAYER_SIZE),
        CollisionLayers::new([Layer::Player], [Layer::Person]),
        AttackTimer {
            timer: Timer::new(Duration::from_millis(ATTACK_SPEED), TimerMode::Repeating),
        },
        Shotgun { ..default() },
        Name::new("Player"),
    ));
}
