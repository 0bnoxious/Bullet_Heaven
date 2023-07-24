pub mod debug;
pub mod global;
pub mod map;
pub mod mob;
pub mod player;
pub mod projectile;

//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;
use bevy::{prelude::*, window::WindowResized};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;
use debug::{draw_collider, move_position};
use global::*;
use map::define_space;
use mob::mob_spawner::InfectedSpawnTimer;
use mob::{
    infected::*,
    mob_spawner::{spawn_infected, spawn_person},
    *,
};
use player::player_spawner::*;
use projectile::{handle_projectile_collision, move_projectile, projectile_spawner::*};
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
        .add_systems(
            Startup,
            (
                setup,
                spawn_player,
                //spawn_person,
                //spawn_infected,
                //define_space,
            ),
        )
        .add_systems(
            Update,
            (
                move_projectile,
                update_mob_velocity,
                infect,
                gamepad_input,
                player_attack,
                update_projectile_lifetime,
                handle_projectile_collision,
                //draw_collider,
                //infected_color,
                move_position,
                target_player,
                move_to_target,
                toggle_resolution,
                spawn_infected,
            ),
        )
        .add_systems(Last, despawn_dead)
        .run()
}

#[derive(Resource)]
struct ResolutionSettings {
    large: Vec2,
    medium: Vec2,
    small: Vec2,
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(RandomDirectionTimer {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });
    commands.insert_resource(InfectedSpawnTimer {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    });

    commands.insert_resource(Gravity(Vec2::ZERO));
    commands.insert_resource(ResolutionSettings {
        large: Vec2::new(1920.0, 1080.0),
        medium: Vec2::new(800.0, 600.0),
        small: Vec2::new(640.0, 360.0),
    });
}

fn despawn_dead(mut query: Query<Entity, With<Dead>>, mut commands: Commands) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

/// This system shows how to request the window to a new resolution
fn toggle_resolution(
    keys: Res<Input<KeyCode>>,
    mut windows: Query<&mut Window>,
    resolution: Res<ResolutionSettings>,
) {
    let mut window = windows.single_mut();

    if keys.just_pressed(KeyCode::Key1) {
        let res = resolution.small;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key2) {
        let res = resolution.medium;
        window.resolution.set(res.x, res.y);
    }
    if keys.just_pressed(KeyCode::Key3) {
        let res = resolution.large;
        window.resolution.set(res.x, res.y);
    }
}
