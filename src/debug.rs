use bevy::prelude::*;
use bevy_xpbd_2d::{
    parry::shape::TypedShape,
    prelude::Position,
    prelude::{Collider, Sensor},
};

use crate::player::{Player, PLAYER_ANTI_MOB_SPAWN_SIZE};

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

pub fn draw_antispawn_zone(mut gizmos: Gizmos, q: Query<&Position, With<Player>>) {
    let player = q.single();
    gizmos.rect_2d(
        Vec2 {
            x: player.x,
            y: player.y,
        },
        0.0,
        Vec2::new(PLAYER_ANTI_MOB_SPAWN_SIZE, PLAYER_ANTI_MOB_SPAWN_SIZE),
        Color::PINK,
    )
}

pub fn move_position(mut q: Query<(&Transform, &mut Position), With<Sensor>>) {
    for (t, mut p) in &mut q {
        p.0.x = t.translation.x;
        p.0.y = t.translation.y;
    }
}
