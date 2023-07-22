pub mod debug;
pub mod global;
pub mod map;
pub mod mob;
pub mod player;
pub mod projectile;

//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;
use global::*;
use map::define_space;
use mob::{infected::*, mob_spawner::spawn_person, *};
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
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            WorldInspectorPlugin::default(),
        ))
        .insert_resource(Gravity(Vec2::ZERO))
        .add_systems(
            Startup,
            (
                setup,
                spawn_player,
                spawn_person,
                spawn_infected,
                define_space,
            ),
        )
        .add_systems(
            Update,
            (
                move_projectile,
                //update_person_velocity,
                infect,
                gamepad_input,
                player_attack,
                update_projectile_lifetime,
                //handle_infected_collision,
                handle_projectile_collision,
                draw_collider,
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

fn despawn_dead(mut query: Query<Entity, With<Dead>>, mut commands: Commands) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}
