use bevy::prelude::*;
use bevy_xpbd_2d::{parry::shape::TypedShape, prelude::Collider};

pub fn draw_collider(mut gizmos: Gizmos, q: Query<(&Collider, &Transform)>) {
    for (colider, t) in &q {
        match colider.as_typed_shape() {
            TypedShape::Cuboid(cube) => {
                let mut aabb: Vec2 = cube.local_aabb().maxs.into();
                aabb *= 2.0;
                gizmos.rect_2d(
                    t.translation.truncate(),
                    0.0,
                    Vec2::new(aabb.x * t.scale.x, aabb.y * t.scale.y),
                    Color::PINK,
                )
            }
            _ => todo!(),
        };
    }
}
