use bevy::{prelude::*, sprite::MaterialMesh2dBundle, transform::commands};
use bevy_xpbd_2d::{
    math::{Scalar, Vector},
    parry::shape::TypedShape,
    prelude::{AngularVelocity, Collider, RayHits, RigidBody},
    prelude::{Position, RayCaster},
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
            println!("skouassa : {rayhits:?}");
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

pub fn render_rays(mut rays: Query<(&mut RayCaster, &mut RayHits)>, mut gizmos: Gizmos) {
    for (ray, hits) in &mut rays {
        // Convert to Vec3 for lines
        let origin = ray.global_origin();
        let direction = ray.global_direction();

        for hit in hits.iter() {
            gizmos.line_2d(
                origin,
                origin + direction * hit.time_of_impact as f32,
                Color::GREEN,
            );
        }
        if hits.is_empty() {
            gizmos.line_2d(origin, origin + direction * 1_000_000.0, Color::ORANGE_RED);
        }
    }
}

pub fn spawn_test_colliders(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 16.0;
    for x in -4..=4 {
        for y in -4..=4 {
            if (-3..4).contains(&x) && (-3..4).contains(&y) {
                continue;
            }

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(radius).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::rgb(0.2, 0.7, 0.9))),
                    transform: Transform::from_xyz(
                        x as f32 * radius * 3.0,
                        y as f32 * radius * 3.0,
                        0.0,
                    ),
                    ..default()
                },
                Collider::ball(radius as Scalar),
            ));
        }
    }

    commands.spawn((
        RigidBody::Kinematic,
        AngularVelocity(0.2),
        RayCaster::new(Vector::ZERO, Vector::X),
    ));
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
