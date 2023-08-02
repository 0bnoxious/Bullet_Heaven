use bevy::prelude::*;
use bevy_xpbd_2d::{math::Vector, prelude::*};

pub mod wave;

pub const BOX_SIZE: f32 = 260.;

#[derive(Component)]
pub struct Wall;

pub fn define_space(mut commands: Commands) {
    let square_sprite = Sprite {
        color: Color::rgb(0.7, 0.7, 0.8),
        custom_size: Some(Vec2::splat(50.0)),
        ..default()
    };

    // Ceiling
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(20.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Position(Vector::Y * 50.0 * 6.0),
        Collider::cuboid(50.0 * 20.0, 50.0),
        Wall,
    ));
    // Floor
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(20.0, 1.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Position(Vector::NEG_Y * 50.0 * 6.0),
        Collider::cuboid(50.0 * 20.0, 50.0),
        Wall,
    ));
    // Left wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite.clone(),
            transform: Transform::from_scale(Vec3::new(1.0, 11.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Position(Vector::NEG_X * 50.0 * 9.5),
        Collider::cuboid(50.0, 50.0 * 11.0),
        Wall,
    ));
    // Right wall
    commands.spawn((
        SpriteBundle {
            sprite: square_sprite,
            transform: Transform::from_scale(Vec3::new(1.0, 11.0, 1.0)),
            ..default()
        },
        RigidBody::Static,
        Position(Vector::X * 50.0 * 9.5),
        Collider::cuboid(50.0, 50.0 * 11.0),
        Wall,
    ));
}
