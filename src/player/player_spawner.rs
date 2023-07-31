use std::time::Duration;

use bevy::prelude::*;
use bevy_xpbd_2d::math::Vector;
use bevy_xpbd_2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::global::*;
use crate::{global::AimType, mob::DetectionZone};

use super::DETECTION_ARRAY_PRECISION;
use super::{player_input::PlayerAction, AttackTimer, Player, ATTACK_SPEED, PLAYER_SIZE};

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
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
            aim_type: AimType::Closest,
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
        DetectionZone {
            raycast_array: build_detection_zone(Vec2 { x: 0., y: 0. }, DETECTION_ARRAY_PRECISION),
        },
        Name::new("player"),
    ));
}

// direction of the raycasts are calculated relative to an
// arbitrary point right above the detection zone
//
//      xiyi _
//      |     \
//      |      ↓
//      |     x'y'
//      | θ /
//      |  /
//     xoyo
//  detection zone center
//  x' ​​= x * cos(θ) - y * sin(θ)
//  y' = y * cos(θ) - x * sin(θ)
pub fn build_detection_zone(origin: Vector, precision: i32) -> Vec<RayCaster> {
    let mut detection_zone: Vec<RayCaster> = Vec::new();

    if precision == 1 {
        let direction = Vector::new(origin.x, &origin.y + 1.);
        detection_zone.push(RayCaster::new(origin, direction));
        return detection_zone;
    }

    let mut number_of_raycasts = 1;

    for _n in 1..precision {
        number_of_raycasts *= 2;
    }

    let angle_between_rays = std::f64::consts::PI * 2. / number_of_raycasts as f64;

    // arbitrary point right above the detection zone
    let ray_direction = Vec2 {
        x: origin.x,
        y: origin.y + 1.,
    };

    let mut ray_num = 1.;
    (0..number_of_raycasts).for_each(|_r| {
        // x' ​​= x * cos(θ) - y * sin(θ)
        // y' = y * cos(θ) - x * sin(θ)
        let direction_x = ray_direction.x * (ray_num * angle_between_rays).cos() as f32
            - ray_direction.y * (ray_num * angle_between_rays).sin() as f32;
        let direction_y = ray_direction.y * (ray_num * angle_between_rays).cos() as f32
            - ray_direction.x * (ray_num * angle_between_rays).sin() as f32;

        ray_num += 1.;

        detection_zone.push(RayCaster::new(
            origin,
            Vec2 {
                x: direction_x,
                y: direction_y,
            },
        ))
    });

    detection_zone
}

pub fn update_detection_zone_position(
    mut querry: Query<(&mut DetectionZone, &Position), With<Player>>,
) {
    for (mut zone, player_pos) in &mut querry {
        for ray in zone.raycast_array.iter_mut() {
            ray.origin = player_pos.0;
        }
    }
}
