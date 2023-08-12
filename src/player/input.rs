use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};

use crate::global::AimType;

use super::spawner::PlayerBundle;
use super::Player;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Up,
    Down,
    Left,
    Right,
    Ability1,
    Ability2,
    Ability3,
    Ability4,
    Ultimate,
}

impl PlayerAction {
    // Lists like this can be very useful for quickly matching subsets of actions
    const DIRECTIONS: [Self; 4] = [
        PlayerAction::Up,
        PlayerAction::Down,
        PlayerAction::Left,
        PlayerAction::Right,
    ];

    pub fn direction(self) -> Option<Direction> {
        match self {
            PlayerAction::Up => Some(Direction::NORTH),
            PlayerAction::Down => Some(Direction::SOUTH),
            PlayerAction::Left => Some(Direction::WEST),
            PlayerAction::Right => Some(Direction::EAST),
            _ => None,
        }
    }
}

impl PlayerBundle {
    pub fn player_input_map() -> InputMap<PlayerAction> {
        // This allows us to replace `ArpgAction::Up` with `Up`,
        // significantly reducing boilerplate
        use PlayerAction::*;
        let mut input_map = InputMap::default();

        // Movement
        input_map.insert(KeyCode::Up, Up);
        input_map.insert(GamepadButtonType::DPadUp, Up);

        input_map.insert(KeyCode::Down, Down);
        input_map.insert(GamepadButtonType::DPadDown, Down);

        input_map.insert(KeyCode::Left, Left);
        input_map.insert(GamepadButtonType::DPadLeft, Left);

        input_map.insert(KeyCode::Right, Right);
        input_map.insert(GamepadButtonType::DPadRight, Right);

        // Abilities
        input_map.insert(KeyCode::Q, Ability1);
        input_map.insert(GamepadButtonType::West, Ability1);
        input_map.insert(MouseButton::Left, Ability1);

        input_map.insert(KeyCode::W, Ability2);
        input_map.insert(GamepadButtonType::North, Ability2);
        input_map.insert(MouseButton::Right, Ability2);

        input_map.insert(KeyCode::E, Ability3);
        input_map.insert(GamepadButtonType::East, Ability3);

        input_map.insert(KeyCode::Space, Ability4);
        input_map.insert(GamepadButtonType::South, Ability4);

        input_map.insert(KeyCode::R, Ultimate);
        input_map.insert(GamepadButtonType::LeftTrigger2, Ultimate);

        input_map
    }
}

#[derive(Event)]
pub struct PlayerWalk {
    pub direction: Direction,
}

#[derive(Event)]
pub struct PlayerAimSwap {
    pub aim: AimType,
}

pub fn player_walks(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut event_writer: EventWriter<PlayerWalk>,
) {
    let action_state = query.single();

    let mut direction_vector = Vec2::ZERO;

    for input_direction in PlayerAction::DIRECTIONS {
        if action_state.pressed(input_direction) {
            if let Some(direction) = input_direction.direction() {
                // Sum the directions as 2D vectors
                direction_vector += Vec2::from(direction);
            }
        }
    }

    // Then reconvert at the end, normalizing the magnitude
    let net_direction: Result<Direction, NearlySingularConversion> = direction_vector.try_into();

    if let Ok(direction) = net_direction {
        event_writer.send(PlayerWalk { direction });
    }
}

pub fn player_swaps_aim(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut player_aim_query: Query<&AimType, With<Player>>,
    mut event_writer: EventWriter<PlayerAimSwap>,
) {
    let action_state = query.single();

    if action_state.just_pressed(PlayerAction::Ability1) {
        let swapped_aim: AimType = player_aim_query.single_mut().next();
        event_writer.send(PlayerAimSwap { aim: swapped_aim });
    }
}
