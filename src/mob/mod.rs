use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::global::*;
use crate::map::BOX_SIZE;
use crate::player::Player;
use crate::projectile::Damage;
use crate::projectile::Projectile;

use self::healthy::*;
use self::infected::*;

pub mod healthy;
pub mod infected;
pub mod spawner;

pub const DEFAULT_MOB_SIZE: f32 = 10.;
pub const DEFAULT_MOB_HP: i32 = 1;
pub const DEFAULT_MOB_DEFENSE: i32 = 0;
pub const DEFAULT_MOB_DAMAGE: i32 = 0;
pub const DEFAULT_MOB_ATTACK_SPEED: f32 = 1.;
pub const DEFAULT_MOB_MOVEMENT_SPEED: f32 = 0.;
pub const DEFAULT_MOB_COLOR: Color = Color::BLACK;

#[derive(Component, Debug)]
pub struct Stats {
    pub hit_points: i32,
    pub movement_speed: f32,
    pub attack_speed: f32,
    pub defense: i32,
    pub damage: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            hit_points: DEFAULT_MOB_HP,
            movement_speed: DEFAULT_MOB_MOVEMENT_SPEED,
            attack_speed: DEFAULT_MOB_ATTACK_SPEED,
            defense: DEFAULT_MOB_DEFENSE,
            damage: DEFAULT_MOB_DAMAGE,
        }
    }
}

#[derive(Component, Debug)]
pub struct Target {
    pub target: Vec2,
}

#[derive(Resource)]
pub struct RandomDirectionTimer {
    pub timer: Timer,
}

#[derive(Bundle)]
pub struct MobBundle {
    sprite_bundle: SpriteBundle,
    rigid_body: RigidBody,
    position: Position,
    collider: Collider,
    damage: Damage,
}

impl Default for MobBundle {
    fn default() -> Self {
        let square_sprite = Sprite {
            color: DEFAULT_MOB_COLOR,
            custom_size: Some(Vec2 {
                x: DEFAULT_MOB_SIZE,
                y: DEFAULT_MOB_SIZE,
            }),
            ..default()
        };

        let mut rng = rand::thread_rng();
        let posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let dmg_vec: Vec<i32> = Vec::new();

        Self {
            sprite_bundle: SpriteBundle {
                sprite: square_sprite,
                transform: Transform::from_translation(Vec3::new(posx, posy, 0.)),
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            position: Position(Vec2 { x: posx, y: posy }),
            collider: Collider::cuboid(DEFAULT_MOB_SIZE, DEFAULT_MOB_SIZE),
            damage: Damage { instances: dmg_vec },
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_mob_velocity(
    mut velocity_query: Query<
        &mut LinearVelocity,
        (Without<Projectile>, Without<Player>, Without<Sensor>),
    >,
    time: Res<Time>,
    mut timer_res: ResMut<RandomDirectionTimer>,
) {
    timer_res.timer.tick(time.delta());

    let mut rng = rand::thread_rng();
    for mut velocity in &mut velocity_query {
        if timer_res.timer.just_finished() {
            let new_velocity = random_velocity(&mut rng);
            velocity.x = new_velocity.x * HEALTHY_MOVEMENT_SPEED;
            velocity.y = new_velocity.y * HEALTHY_MOVEMENT_SPEED;
        }
    }
}
