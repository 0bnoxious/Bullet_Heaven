use bevy::prelude::*;
use bevy_xpbd_2d::{
    parry::shape::TypedShape,
    prelude::Position,
    prelude::{Collider, Sensor},
};

use crate::{
    player::{Player, DEFAULT_PLAYER_ANTI_MOB_SPAWN_SIZE},
    projectile::Projectile,
    targeting::{define_spread, HasTarget},
    weapon::shotgun::Shotgun,
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

pub fn draw_antispawn_zone(mut gizmos: Gizmos, q: Query<&Position, With<Player>>) {
    let player = q.single();
    gizmos.rect_2d(
        Vec2 {
            x: player.x,
            y: player.y,
        },
        0.0,
        Vec2::new(
            DEFAULT_PLAYER_ANTI_MOB_SPAWN_SIZE,
            DEFAULT_PLAYER_ANTI_MOB_SPAWN_SIZE,
        ),
        Color::PINK,
    )
}

pub fn move_position(mut q: Query<(&Transform, &mut Position), With<Sensor>>) {
    for (t, mut p) in &mut q {
        p.0.x = t.translation.x;
        p.0.y = t.translation.y;
    }
}

pub fn draw_player_target_line(
    mut gizmos: Gizmos,
    mut q: Query<(&HasTarget, &Position), With<Player>>,
) {
    for (t, p) in &mut q {
        gizmos.line_2d(p.0, t.target_position, Color::ORANGE_RED);
    }
}

#[allow(clippy::type_complexity)]
pub fn draw_weapon_spread_lines(
    mut gizmos: Gizmos,
    mut query: Query<(&HasTarget, &Position, &Shotgun), (With<Player>, Without<Projectile>)>,
) {
    for (player_has_target, player_position, gun) in &mut query {
        let distance_to_target =
            Vec2::distance(player_position.0, player_has_target.target_position);

        let left = Vec2::from_angle((gun.spread).to_radians())
            .rotate(player_has_target.target_position - player_position.0);
        let right = Vec2::from_angle(-(gun.spread).to_radians())
            .rotate(player_has_target.target_position - player_position.0);

        let spread = define_spread(
            player_position.0,
            player_has_target.target_position,
            gun.spread,
        );

        println!("spread: {spread:?}");

        gizmos.line_2d(
            player_position.0,
            spread * distance_to_target,
            Color::FUCHSIA,
        );

        gizmos.line_2d(
            player_position.0,
            player_has_target.target_position,
            Color::ORANGE_RED,
        );
        gizmos.line_2d(player_position.0, left * distance_to_target, Color::WHITE);
        gizmos.line_2d(player_position.0, right * distance_to_target, Color::WHITE);
    }
}
