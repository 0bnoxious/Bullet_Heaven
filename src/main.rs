pub mod global;
pub mod mobs;
pub mod player;
pub mod projectile;

//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use global::*;
use mobs::{
    infected::{infect, spawn_infected},
    person::*,
    PERSONSIZE,
};
use player::player_spawner::*;
use projectile::projectile_spawner::*;
use std::time::Duration;

fn main() {
    App::new()
        /* .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))*/
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .add_systems(Startup, (setup, spawn_player, spawn_person, spawn_infected))
        .add_systems(
            Update,
            (
                move_population,
                move_projectile,
                update_person_direction,
                infect,
                define_space,
                gamepad_input,
                player_attack,
                update_projectile_lifetime,
                collide_projectile,
            ),
        )
        .add_systems(Last, despawn_dead)
        .run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(PersonDirectionTimer {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });
}

#[derive(Component)]
struct Dead;

fn define_space(mut query: Query<&mut Transform, Without<Projectile>>) {
    let minxy = (-BOXSIZE / 2.) - PERSONSIZE / 2.;
    let maxxy = (BOXSIZE / 2.) - PERSONSIZE / 2.;

    for mut transform in query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < minxy {
            translation.x = minxy;
        } else if translation.x > maxxy {
            translation.x = maxxy
        }
        if translation.y < minxy {
            translation.y = minxy;
        } else if translation.y > maxxy {
            translation.y = maxxy
        }

        transform.translation = translation
    }
}

fn despawn_dead(mut query: Query<Entity, With<Dead>>, mut commands: Commands) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
