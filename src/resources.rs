use bevy::prelude::*;

#[derive(Resource)]
pub enum AimType {
    Random,
    Closest,
    HomingClosest,
    Mouse,
    HomingMouse,
    Direction,
}
