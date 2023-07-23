use bevy::prelude::*;
use bevy_xpbd_2d::prelude::{Collider, LockedAxes, Position, RigidBody};
use rand::Rng;

pub const INFECTED_HP: i32 = 3;
pub const INFECTION_ODDS: i32 = 5; // 1 in x chance to infect
pub const INFECTED_COLOR: Color = Color::RED;

use super::*;
#[derive(Component, Debug)]
pub struct Infected;

#[derive(Bundle)]
pub struct InfectedBundle {
    infected: Infected,
    sprite_bundle: SpriteBundle,
    stats: Stats,
    layer: CollisionLayers,
}

impl Default for InfectedBundle {
    fn default() -> Self {
        let square_sprite = Sprite {
            color: INFECTED_COLOR,
            custom_size: Some(Vec2 {
                x: DEFAULT_MOB_SIZE,
                y: DEFAULT_MOB_SIZE,
            }),
            ..default()
        };

        let mut rng = rand::thread_rng();
        let posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);

        Self {
            infected: Infected,
            sprite_bundle: SpriteBundle {
                sprite: square_sprite,
                transform: Transform::from_translation(Vec3::new(posx, posy, 0.)),
                ..default()
            },
            layer: CollisionLayers::new(
                [Layer::Infected],
                [Layer::Player, Layer::Projectile, Layer::Infected],
            ),
            stats: Stats {
                hit_points: INFECTED_HP,
            },
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn infect(
    mut commands: Commands,
    query_infected: Query<&Position, With<Infected>>,
    mut query_healthy: Query<
        (Entity, &Position, &mut Sprite, &mut InfectionAttemptTimer),
        (With<Person>, Without<Infected>),
    >,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for infected_position in &query_infected {
        for (entity, healthy_position, _, mut infect_timer) in &mut query_healthy {
            let distance = infected_position.distance(Vec2 {
                x: healthy_position.x,
                y: healthy_position.y,
            });
            if distance < DEFAULT_MOB_SIZE {
                //attempt to infect once every INFECTION_ATTEMPT_DELAY_MS milliseconds
                infect_timer.timer.tick(time.delta());
                if infect_timer.timer.finished() {
                    // 1/INFECTION_ODDS chance to infect
                    if rng.gen_range(0..INFECTION_ODDS) == 0 {
                        commands.entity(entity).despawn_recursive();

                        let square_sprite = Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2 {
                                x: DEFAULT_MOB_SIZE,
                                y: DEFAULT_MOB_SIZE,
                            }),
                            ..default()
                        };
                        commands.spawn((
                            Person,
                            RigidBody::Dynamic,
                            Position(Vec2::new(infected_position.x, infected_position.y)),
                            LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED),
                            Collider::cuboid(DEFAULT_MOB_SIZE, DEFAULT_MOB_SIZE),
                            LockedAxes::ROTATION_LOCKED,
                            InfectedBundle::default(),
                        ));
                    }
                }
            }
        }
    }
}

// TODO use change detection
/*pub fn infected_color(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Sprite, &mut CollisionLayers), Added<Infected>>,
) {
    for (e, mut sprite, mut layers) in &mut q {
        sprite.color = Color::RED;
        *layers = CollisionLayers::new(
            [Layer::Infected],
            [Layer::Player, Layer::Projectile, Layer::Infected],
        );
        commands
            .entity(e)
            .insert((Stats::default(), InfectAttemptTimer::default()));
    }
}*/
