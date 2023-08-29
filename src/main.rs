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
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;
use debug::DebugPlugin;
use global::*;

use map::MapPlugin;
use mob::MobPlugin;
use player::{spawner::*, PlayerPlugin};
use projectile::ProjectilePlugin;
use targeting::TargetingPlugin;
use ui::{hud::wave_timer::update_hud_wave_timer_value, settings::SettingsPlugin, KayakUiPlugin};
use weapon::WeaponPlugin;

fn main() {
    App::new()
        .insert_resource(SubstepCount(2))
        .insert_resource(Gravity(Vec2::ZERO))
        .add_plugins((
            DefaultPlugins.set(set_primary_window()),
            PhysicsPlugins::default(),
            WorldInspectorPlugin::default(),
            MapPlugin,
            PlayerPlugin,
            ProjectilePlugin,
            TargetingPlugin,
            WeaponPlugin,
            MobPlugin,
            KayakUiPlugin,
            //DebugPlugin,
            SettingsPlugin,
        ))
        .add_systems(
            Update,
            (
                resolve_damage.before(respawn_player),
                update_hud_wave_timer_value,
            ),
        )
        .add_systems(Last, despawn_dead)
        .run()
}
