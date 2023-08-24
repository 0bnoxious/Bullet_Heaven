pub mod debug;
pub mod global;
pub mod map;
pub mod mob;
pub mod player;
pub mod projectile;
pub mod targeting;
pub mod ui;
pub mod weapon;

use bevy::prelude::*;
use bevy::window::{PresentMode, WindowTheme};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;
use debug::DebugPlugin;
use global::*;
use kayak_ui::prelude::{widgets::*, *};
use map::MapPlugin;
use mob::MobPlugin;
use player::{spawner::*, PlayerPlugin};
use projectile::ProjectilePlugin;
use targeting::TargetingPlugin;
use ui::hud::setup_hud;
use weapon::WeaponPlugin;

fn main() {
    App::new()
        .insert_resource(SubstepCount(2))
        .add_plugins((
            DefaultPlugins.set(set_primary_window()),
            PhysicsPlugins::default(),
            WorldInspectorPlugin::default(),
            KayakContextPlugin,
            KayakWidgets,
            MapPlugin,
            PlayerPlugin,
            ProjectilePlugin,
            TargetingPlugin,
            WeaponPlugin,
            MobPlugin,
            DebugPlugin,
        ))
        .add_systems(Startup, (setup_resolution, setup_hud))
        .add_systems(
            Update,
            (toggle_resolution, resolve_damage.before(respawn_player)),
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

pub fn setup_resolution(mut commands: Commands) {
    commands.insert_resource(Gravity(Vec2::ZERO));
    commands.insert_resource(ResolutionSettings {
        large: Vec2::new(1920.0, 1080.0),
        medium: Vec2::new(800.0, 600.0),
        small: Vec2::new(640.0, 360.0),
    });
}

pub fn set_primary_window() -> WindowPlugin {
    WindowPlugin {
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
    }
}

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
