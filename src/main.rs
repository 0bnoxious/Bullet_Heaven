pub mod debug;
pub mod global;
pub mod map;
pub mod mob;
pub mod player;
pub mod projectile;
pub mod targeting;
pub mod weapon;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;

use debug::egui::{
    initialize_uistate, toggle_rifle, toggle_shotgun, ui_example_system, update_player_rifle_stats,
    update_player_shotgun_stats, update_player_stats, UiState,
};
use debug::gizmo::draw_weapon_spread_lines;
use global::*;
use leafwing_input_manager::prelude::*;
use map::define_space;
use map::wave::{manage_waves, spawn_waves_manager};
use mob::spawner::SpawnTimer;
use player::action::{move_player, swap_player_aim};
use player::input::{player_swaps_aim, player_walks, PlayerAction, PlayerAimSwap, PlayerWalk};
use player::{
    spawner::*, update_player_rifle_cooldown, update_player_shotgun_cooldown,
    PlayerRifleCoolDownChange, PlayerShotGunCoolDownChange,
};
use projectile::movement::{move_rifle_projectile, move_shotgun_projectile};
use projectile::{handle_projectile_collision, spawner::*};
use std::time::Duration;
use targeting::{move_mob_to_target, target_enemy, target_player, HasTarget};
use weapon::rifle::fire_rifle;
use weapon::shotgun::fire_shotgun;

fn main() {
    App::new()
        /* .add_plugins((
            DefaultPlugins,
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin::default(),
        ))*/
        .insert_resource(SubstepCount(3))
        .init_resource::<UiState>()
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
            //EguiPlugin,
        ))
        .add_systems(
            Startup,
            (
                setup,
                spawn_player,
                define_space,
                spawn_waves_manager,
                initialize_uistate,
            ),
        )
        .add_systems(
            Update,
            (
                update_projectile_lifetime,
                handle_projectile_collision,
                target_player,
                move_mob_to_target,
                toggle_resolution,
                apply_damage,
                manage_waves,
                move_player,
                //swap_player_aim,
                fire_rifle,
                fire_shotgun,
                target_enemy,
                move_shotgun_projectile,
                move_rifle_projectile,
                //debug egui ############################################
                update_player_stats,
                update_player_shotgun_cooldown,
                update_player_rifle_cooldown,
                toggle_shotgun,
                toggle_rifle,
                //debug guizmo ############################################
                //move_projectile_to_target,
                //draw_collider,
                //draw_antispawn_zone,
                //draw_player_target_line,
                //draw_weapon_spread_lines,
            ),
        )
        .add_systems(Update, player_walks)
        .add_systems(Update, player_swaps_aim)
        .add_systems(Update, ui_example_system)
        .add_systems(Update, update_player_shotgun_stats)
        .add_systems(Update, update_player_rifle_stats)
        .add_event::<PlayerWalk>()
        .add_event::<PlayerAimSwap>()
        .add_event::<PlayerShotGunCoolDownChange>()
        .add_event::<PlayerRifleCoolDownChange>()
        .add_systems(Last, despawn_dead)
        .register_type::<HasTarget>()
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
