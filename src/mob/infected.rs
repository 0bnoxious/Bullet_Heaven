use bevy::prelude::*;
use bevy_xpbd_2d::prelude::{Collider, LockedAxes, Position, RigidBody};
use rand::Rng;

use super::*;

#[derive(Bundle)]
pub struct InfectedBundle {
    infected: Infected,
    stats: Stats,
    infect_timer: InfectTimer,
    layer: CollisionLayers,
}

impl Default for InfectedBundle {
    fn default() -> Self {
        Self {
            infected: Infected,
            stats: Default::default(),
            infect_timer: Default::default(),
            layer: CollisionLayers::new(
                [Layer::Infected],
                [Layer::Player, Layer::Projectile, Layer::Infected],
            ),
        }
    }
}

#[derive(Component, Debug)]
pub struct Infected;

pub fn spawn_infected(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    let square_sprite = Sprite {
        color: Color::rgb(1., 0., 0.),
        custom_size: Some(Vec2 {
            x: PERSON_SIZE,
            y: PERSON_SIZE,
        }),
        ..default()
    };

    let mut v = vec![];
    for _ in 0..INFECTED_COUNT {
        let posx = rng.gen_range(-BOX_SIZE..=BOX_SIZE);
        let posy = rng.gen_range(-BOX_SIZE..=BOX_SIZE);

        v.push((
            Person,
            SpriteBundle {
                sprite: square_sprite.clone(),
                transform: Transform::from_translation(Vec3::new(posx, posy, 0.)),
                ..default()
            },
            RigidBody::Dynamic,
            Position(Vec2::new(posx, posy)),
            LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED),
            Collider::cuboid(PERSON_SIZE, PERSON_SIZE),
            LockedAxes::ROTATION_LOCKED,
            InfectedBundle::default(),
        ));
    }
    commands.spawn_batch(v);
}

#[allow(clippy::type_complexity)]
pub fn infect(
    mut commands: Commands,
    query_infected: Query<&Position, With<Infected>>,
    mut query_healthy: Query<
        (Entity, &Position, &mut Sprite, &mut InfectTimer),
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
            if distance < PERSON_SIZE {
                //attempt to infect once every 1/5 second
                infect_timer.timer.tick(time.delta());
                if infect_timer.timer.finished() {
                    // 1/5 chance to infect
                    if rng.gen_range(0..5) == 0 {
                        commands.entity(entity).despawn_recursive();

                        let square_sprite = Sprite {
                            color: Color::rgb(1., 0., 0.),
                            custom_size: Some(Vec2 {
                                x: PERSON_SIZE,
                                y: PERSON_SIZE,
                            }),
                            ..default()
                        };
                        commands.spawn((
                            Person,
                            SpriteBundle {
                                sprite: square_sprite.clone(),
                                transform: Transform::from_translation(Vec3::new(
                                    infected_position.x,
                                    infected_position.y,
                                    0.,
                                )),
                                ..default()
                            },
                            RigidBody::Dynamic,
                            Position(Vec2::new(infected_position.x, infected_position.y)),
                            LinearVelocity(random_velocity(&mut rng).truncate() * PERSON_SPEED),
                            Collider::cuboid(PERSON_SIZE, PERSON_SIZE),
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
pub fn infected_color(
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
            .insert((Stats::default(), InfectTimer::default()));
    }
}
