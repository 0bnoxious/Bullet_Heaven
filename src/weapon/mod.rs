use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_xpbd_2d::prelude::Position;

use crate::{global::Target, mob::infected::Infected, player::Player};

pub const DEFAULT_WEAPON_DAMAGE: f64 = 1.;
pub const DEFAULT_WEAPON_FIRE_RATE: f64 = 1000.;
pub const DEFAULT_WEAPON_RANGE: f64 = 100.;
pub const DEFAULT_WEAPON_SPREAD: f64 = 1.;

pub mod shotgun;

#[derive(Component)]
pub struct Weapon {
    pub damage: f64,
    pub fire_rate: f64,
    pub range: f64,
    pub spread: f64,
}

pub fn default_weapon() -> Weapon {
    Weapon {
        range: DEFAULT_WEAPON_DAMAGE,
        damage: DEFAULT_WEAPON_FIRE_RATE,
        fire_rate: DEFAULT_WEAPON_RANGE,
        spread: DEFAULT_WEAPON_SPREAD,
    }
}

#[derive(SystemParam)]
pub struct ClosestTarget<'w, 's> {
    infected_query: Query<'w, 's, &'static Position, (With<Infected>, Without<Player>)>,
    player_query: Query<'w, 's, &'static Position, (With<Player>, Without<Infected>)>,
}

impl<'w, 's> ClosestTarget<'w, 's> {
    pub fn infected(&mut self) -> Target {
        let player_position = self.player_query.single();
        let mut closest_dist = f32::MAX;
        let mut closest_pos = Target {
            position: Position(Vec2::ZERO),
        };

        for infected_pos in self.infected_query.iter() {
            let distance = Vec2::distance(player_position.0, infected_pos.0);
            if distance < closest_dist {
                closest_dist = distance;
                let target = Target {
                    position: Position(Vec2 {
                        x: infected_pos.x,
                        y: infected_pos.y,
                    }),
                };
                closest_pos = target;
            }
        }
        closest_pos
    }
}
