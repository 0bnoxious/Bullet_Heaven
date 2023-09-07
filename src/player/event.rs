use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::{errors::NearlySingularConversion, orientation::Direction};

use crate::global::AimType;

use super::input::PlayerAction;
use super::Player;

#[derive(Event)]
pub struct PlayerWalkEvent {
    pub direction: Direction,
}

#[derive(Event)]
pub struct PlayerAimSwapEvent {
    pub aim: AimType,
}

pub fn player_walked(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut event_writer: EventWriter<PlayerWalkEvent>,
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
        event_writer.send(PlayerWalkEvent { direction });
    }
}

pub fn player_swapped_aim(
    query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut player_aim_query: Query<&AimType, With<Player>>,
    mut event_writer: EventWriter<PlayerAimSwapEvent>,
) {
    let action_state = query.single();

    if action_state.just_pressed(PlayerAction::Ability1) {
        let swapped_aim: AimType = player_aim_query.single_mut().next();
        event_writer.send(PlayerAimSwapEvent { aim: swapped_aim });
    }
}
