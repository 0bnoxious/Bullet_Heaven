use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    mob::{
        infected::{Infected, DEFAULT_INFECTED_MOVEMENT_SPEED},
        Mob,
    },
    player::Player,
};

#[derive(Component)]
pub struct Target;

#[derive(Component, Clone, Copy, Debug)]
pub struct HasTarget {
    pub target_position: Vec2,
}

//commence par infected gros bs
/*pub fn add_unit_target(
    commands: Commands,
    can_target_query: Query<Entity, (With<Target>, With<Mob>)>,
    has_target_query: Query<Entity, Without<HasTarget>>,
) {
}*/

pub fn player_targeting(
    mut commands: Commands,
    infected_as_target_querry: Query<&Target, With<Infected>>,
    mut has_target_query: Query<Entity, (With<Player>, Without<HasTarget>)>,
    mut target_pos_query: Query<&mut HasTarget, With<Player>>,
    mut closest: ClosestTarget,
) {
    // do not assign target when there are no enemies
    if !infected_as_target_querry.is_empty() {
        // when the player have no target
        if target_pos_query.is_empty() {
            for player_entity_without_target in &mut has_target_query {
                let closest = closest.infected();
                //println!("assigning {closest:?} as player target!");
                commands
                    .entity(player_entity_without_target)
                    .insert(HasTarget {
                        target_position: closest,
                    });
            }
        } else {
            for mut target in &mut target_pos_query {
                target.target_position = closest.infected();
                //println!("updating {} as player target!", target.target_position);
            }
        }
    }
}

pub fn move_unit_to_target(
    mut infected_query: Query<
        (&mut LinearVelocity, &Position, &HasTarget),
        (With<Mob>, Without<Player>),
    >,
) {
    for (mut velocity, position, target) in &mut infected_query {
        // get the vector from the infected to the target and normalise it.
        let to_player = (target.target_position - position.0).normalize();

        velocity.x = to_player.x * DEFAULT_INFECTED_MOVEMENT_SPEED;
        velocity.y = to_player.y * DEFAULT_INFECTED_MOVEMENT_SPEED;
    }
}

pub fn target_player(
    mut commands: Commands,
    player_quary: Query<&Position, With<Player>>,
    mut infected_querry: Query<Entity, (With<Infected>, Without<HasTarget>)>,
) {
    let player_position: Vec2 = player_quary.single().0;

    for infected_entity in infected_querry.iter_mut() {
        commands.entity(infected_entity).insert(HasTarget {
            target_position: player_position,
        });
    }
}

#[derive(SystemParam)]
pub struct ClosestTarget<'w, 's> {
    infected_query: Query<'w, 's, &'static Position, (With<Infected>, Without<Player>)>,
    player_query: Query<'w, 's, &'static Position, (With<Player>, Without<Infected>)>,
}

impl<'w, 's> ClosestTarget<'w, 's> {
    pub fn infected(&mut self) -> Vec2 {
        let player_position = self.player_query.single();
        let mut closest_dist = f32::MAX;
        let mut closest_pos = Vec2::ZERO;

        for infected_pos in self.infected_query.iter() {
            let distance = Vec2::distance(player_position.0, infected_pos.0);
            if distance < closest_dist {
                closest_dist = distance;
                closest_pos = infected_pos.0;
            }
        }
        closest_pos
    }
}

pub fn define_spread(origin: Vec2, direction: Vec2, spread: f32) -> Vec2 {
    if spread <= 0. {
        return direction;
    }
    let mut rng = rand::thread_rng();

    //determine deviation from target using a bell curve type distribution
    let deviation = rng.gen_range(0.0..spread) + rng.gen_range(0.0..spread) - spread;

    //rotate the target vector by the deviation
    //let (y, x) = (deviation + std::f32::consts::PI / 2.0).sin_cos();
    let skoica = Vec2::from_angle(deviation.to_radians())
        .rotate(direction - origin)
        .normalize();
    print!("skoica: {skoica:?}");
    skoica

    //let deviation_as_vec = Vec2::new(x as f32, y as f32).normalize();
    //(direction - origin).normalize() + deviation_as_vec

    //rotate the target vector by the deviation
    //let (y, x) = (deviation + std::f32::consts::PI / 2.0).sin_cos();
}
