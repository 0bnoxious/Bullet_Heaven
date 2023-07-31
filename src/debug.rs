use bevy::prelude::*;
use bevy_xpbd_2d::{
    parry::shape::TypedShape,
    prelude::Position,
    prelude::{Collider, RayHits},
};

use crate::{
    global::AimType,
    mob::DetectionZone,
    player::{player_input::PlayerAimSwap, Player},
};

pub fn draw_collider(mut gizmos: Gizmos, q: Query<(&Collider, &Position)>) {
    for (colider, pos) in &q {
        match colider.as_typed_shape() {
            TypedShape::Cuboid(cube) => {
                let mut aabb: Vec2 = cube.local_aabb().maxs.into();
                aabb *= 2.0;
                gizmos.rect_2d(pos.0, 0.0, Vec2::new(aabb.x, aabb.y), Color::PINK)
            }
            _ => todo!(),
        };
    }
}

pub fn render_detection_zones(
    mut rays: Query<&mut DetectionZone>,
    rayhits: Query<&mut RayHits>,
    mut gizmos: Gizmos,
) {
    for zone in &mut rays {
        for ray in zone.raycast_array.iter() {
            //println!("skouassa : {rayhits:?}");
            /*for hits in rayhits.iter() {
                gizmos.line_2d(
                    origin,
                    origin + direction * ray.max_time_of_impact,
                    Color::GREEN,
                );

                if hits.is_empty() {}
            }*/
            /*println!(
                "rendering ray with origin : {}  and direction {}",
                origin,
                origin + direction
            );*/

            gizmos.line_2d(
                ray.origin,
                ray.origin + ray.direction * 500.0,
                Color::ORANGE_RED,
            );
        }
    }
}

pub fn display_player_aim(
    mut player_aim_swap_events: EventReader<PlayerAimSwap>,
    aim_query: Query<&AimType, With<Player>>,
) {
    for _ in player_aim_swap_events.iter() {
        for mut aimtype in &mut aim_query.iter() {
            let kosseca = aimtype;
            println!("PLAYER AIM BEFORE SWAP: {kosseca:?}");
            let next_aim = aimtype.next();
            aimtype = &next_aim;
            let kosseca2 = aimtype;
            println!("PLAYER AIM AFTER SWAP : {kosseca2:?}");
        }
    }
}
