pub mod debug;
pub mod global;
pub mod map;
pub mod mob;
pub mod player;
pub mod projectile;

//use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;
use debug::draw_antispawn_zone;
use global::*;
use leafwing_input_manager::prelude::*;
use map::define_space;
use map::wave::{manage_waves, spawn_waves_manager};
use mob::spawner::SpawnTimer;
use mob::{infected::*, spawner::spawn_infected, *};
use player::player_input::{
    player_swaps_aim, player_walks, PlayerAction, PlayerAimSwap, PlayerWalk,
};
use player::{move_player, player_attack, player_spawner::*, swap_player_aim};
use projectile::{handle_projectile_collision, move_projectile, projectile_spawner::*};
use std::time::Duration;

fn main() {
    App::new()
        /* .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))*/
        .insert_resource(SubstepCount(6))
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bullet Heaven".into(),
                    resolution: (1920., 1080.).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm to resize the window according to the available canvas
                    fit_canvas_to_parent: true,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            WorldInspectorPlugin::default(),
            InputManagerPlugin::<PlayerAction>::default(),
        ))
        .add_systems(
            Startup,
            (setup, spawn_player, define_space, spawn_waves_manager),
        )
        .add_systems(
            Update,
            (
                move_projectile,
                update_mob_velocity,
                player_attack,
                update_projectile_lifetime,
                handle_projectile_collision,
                target_player,
                move_to_target,
                toggle_resolution,
                //spawn_infected,
                apply_damage,
                manage_waves,
                //debug
                //draw_collider,
                move_player,
                swap_player_aim,
                draw_antispawn_zone,
            ),
        )
        .add_systems(Update, player_walks)
        .add_systems(Update, player_swaps_aim)
        .add_event::<PlayerWalk>()
        .add_event::<PlayerAimSwap>()
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
    commands.insert_resource(SpawnTimer {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });
    commands.insert_resource(Gravity(Vec2::ZERO));
    commands.insert_resource(ResolutionSettings {
        large: Vec2::new(1920.0, 1080.0),
        medium: Vec2::new(800.0, 600.0),
        small: Vec2::new(640.0, 360.0),
    });
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
