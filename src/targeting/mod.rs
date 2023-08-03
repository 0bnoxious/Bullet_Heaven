use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_xpbd_2d::prelude::*;

use crate::{
    mob::{
        infected::{Infected, DEFAULT_INFECTED_MOVEMENT_SPEED},
        Mob,
    },
    player::Player,
};

#[derive(Component)]
pub struct Target;

#[derive(Component, Clone, Copy)]
pub struct HasTarget {
    pub target_position: Vec2,
}

//commence par infected gros bs
pub fn add_unit_target(
    commands: Commands,
    can_target_query: Query<Entity, (With<Target>, With<Mob>)>,
    has_target_query: Query<Entity, Without<HasTarget>>,
) {
}

pub fn player_targeting(
    mut commands: Commands,
    infected_as_target_querry: Query<&Target, With<Infected>>,
    test_querry: Query<&Infected>,
    mut has_target_query: Query<Entity, (With<Player>, Without<HasTarget>)>,
    mut closest: ClosestTarget,
) {
    let targetcount = infected_as_target_querry.iter().count();
    println!("target count : {targetcount}");
    if !infected_as_target_querry.is_empty() {
        for player_entity_without_target in &mut has_target_query {
            let closest = closest.infected();
            println!("assigning {closest:?} as player target!");
            commands
                .entity(player_entity_without_target)
                .insert(HasTarget {
                    target_position: closest,
                });
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
