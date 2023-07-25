use std::time::Duration;

use bevy::input::gamepad::GamepadButton;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::global::AimType;
use crate::global::*;

use super::{AttackTimer, Player, ATTACK_SPEED, PLAYER_SIZE, PLAYER_SPEED};

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
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
        Player {
            aim_type: AimType::Random,
        },
    ));
}

// TODO: leafwing-input-manager
pub fn gamepad_input(
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<&mut Position, With<Player>>,
    gamepads: Res<Gamepads>,
) {
    let Some(gamepad) = gamepads.iter().next() else {
        return;
    };

    let up_dpad = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadUp,
    };
    let down_dpad = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadDown,
    };
    let left_dpad = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadLeft,
    };
    let right_dpad = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadRight,
    };

    if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
        info!("{:?} just pressed South", gamepad);
    }

    for mut position in &mut query {
        let mut direction = Vec3::ZERO;

        if buttons.pressed(up_dpad) {
            direction += Vec3::new(0., 1., 0.)
        }

        if buttons.pressed(down_dpad) {
            direction += Vec3::new(0., -1., 0.)
        }

        if buttons.pressed(left_dpad) {
            direction += Vec3::new(-1., 0., 0.)
        }

        if buttons.pressed(right_dpad) {
            direction += Vec3::new(1., 0., 0.)
        }

        position.0 += direction.truncate() * PLAYER_SPEED;
    }
}
