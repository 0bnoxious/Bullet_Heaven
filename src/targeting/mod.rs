use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_xpbd_2d::prelude::*;
use rand::Rng;

use crate::{
    mob::{
        infected::{Infected, DEFAULT_INFECTED_MOVEMENT_SPEED},
        Mob,
    },
    player::Player,
    projectile::Projectile,
};

#[derive(Component, Reflect, Debug)]
pub struct Target;

#[derive(Component, Clone, Copy, Reflect, Debug)]
pub struct HasTarget {
    pub target_position: Vec2,
}

pub fn target_enemy(
    mut commands: Commands,
    infected_as_target_querry: Query<&Target, With<Infected>>,
    mut has_target_query: Query<Entity, (With<Player>, Without<HasTarget>)>,
    mut target_pos_query: Query<&mut HasTarget, (With<Player>, Without<Projectile>)>,
    mut closest: ClosestTarget,
) {
    // do not assign target when there are no enemies
    if !infected_as_target_querry.is_empty() {
        // when the player have no target
        if target_pos_query.is_empty() {
            for player_entity_without_target in &mut has_target_query {
                let closest = closest.infected();
                commands
                    .entity(player_entity_without_target)
                    .insert(HasTarget {
                        target_position: closest,
                    });
            }
        } else {
            for mut target in &mut target_pos_query {
                target.target_position = closest.infected();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn move_mob_to_target(
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
    player_query: Query<&Position, With<Player>>,
    mut infected_without_target_query: Query<Entity, (With<Infected>, Without<HasTarget>)>,
    mut infected_target_query: Query<&mut HasTarget, With<Infected>>,
) {
    let player_position: Vec2 = player_query.single().0;

    for infected_entity in infected_without_target_query.iter_mut() {
        commands.entity(infected_entity).insert(HasTarget {
            target_position: player_position,
        });
    }
    for mut infected_target in &mut infected_target_query {
        infected_target.target_position = player_position;
    }
}

#[allow(clippy::type_complexity)]
#[derive(SystemParam)]
pub struct ClosestTarget<'w, 's> {
    infected_query:
        Query<'w, 's, &'static Position, (With<Infected>, Without<Player>, Without<Projectile>)>,
    player_query: Query<'w, 's, &'static Position, (With<Player>, Without<Infected>)>,
}

impl<'w, 's> ClosestTarget<'w, 's> {
    pub fn infected(&mut self) -> Vec2 {
        let mut closest_dist = f32::MAX;
        let mut closest_pos = Vec2::ZERO;
        for player_position in &mut self.player_query {
            for infected_pos in self.infected_query.iter() {
                let distance = Vec2::distance(player_position.0, infected_pos.0);
                if distance < closest_dist {
                    closest_dist = distance;
                    closest_pos = infected_pos.0;
                }
            }
        }
        closest_pos
    }
}

// faudrait checker les maths qui sont fait icite pass spo claire...
//
// j'ai pas trouvé de fonction pour rotate autour d'un point spécifié MAIS
// from_angle + rotate crée un vecteur à (-1,0) et rotate un point autour de (-1,0)
//
// voici ce que "JE PENSE" que j'ai fait =/
//
// #1 - change manuellement le point (-1,0) pour celui de la position du player
//
// #2 - applique un rotation de degré random
//
// #3 le vecteur que ca donne après la rotation j'ai 0 idée c'est quoi, mais quand
// on multiplie par la distance et qu'on normalise, le point va être alignée comme
// voulu dans le range du spread
pub fn define_spread(origin: Vec2, direction: Vec2, spread: f32) -> Vec2 {
    if spread <= 0. {
        return direction;
    }
    let mut rng = rand::thread_rng();

    //determine deviation from target using a bell curve type distribution
    let deviation = rng.gen_range(0.0..spread) + rng.gen_range(0.0..spread) - spread;

    //rotate the target vector by the deviation
    //                                                  #2                            #1
    let deviated_direction = Vec2::from_angle(deviation.to_radians()).rotate(direction - origin);

    //                                 #3
    (deviated_direction * Vec2::distance(origin, direction)).normalize()
}
