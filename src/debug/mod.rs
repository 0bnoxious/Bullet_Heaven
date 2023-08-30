use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiUserTextures};
use bevy_xpbd_2d::prelude::LinearVelocity;

use crate::{
    global::Stats,
    map::wave::{WaveEnemyCountChange, WaveTimerChange},
    player::Player,
    projectile::Projectile,
    targeting::HasTarget,
    weapon::{rifle::Rifle, Weapon, WeaponCoolDown},
};

use self::egui::*;

pub mod egui;
pub mod gizmo;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UiState>()
            .add_plugins(EguiPlugin)
            .add_systems(Startup, initialize_uistate)
            .add_systems(
                Update,
                (
                    debug_ui_system,
                    update_player_stats,
                    toggle_rifle,
                    update_player_rifle_cooldown,
                    toggle_shotgun,
                    update_player_shotgun_cooldown,
                    update_player_shotgun_stats,
                    update_player_rifle_stats,
                    update_wave_timer,
                    update_enemy_count,
                    //GIZMO ##########################################
                    //move_projectile_to_target,
                    //draw_collider,
                    //draw_antispawn_zone,
                    //draw_player_target_line,
                    //draw_weapon_spread_lines,
                ),
            )
            .add_event::<PlayerRifleCoolDownChange>()
            .add_event::<PlayerShotGunCoolDownChange>()
            .add_event::<WaveTimerChange>()
            .add_event::<WaveEnemyCountChange>();
    }
}

#[allow(clippy::type_complexity)]
pub fn move_projectile_to_target(
    mut projectile_velocity_query: Query<&mut LinearVelocity, With<Projectile>>,
    projectile_target_query: Query<&HasTarget, With<Projectile>>,
) {
    for mut velocity in &mut projectile_velocity_query {
        if velocity.0 == Vec2::ZERO {
            for target in projectile_target_query.iter() {
                velocity.0 = target.target_position * 500.
            }
        }
    }
}

pub fn log_player_hitpoint(player_stats_query: Query<&Stats, With<Player>>) {
    for player_stats in player_stats_query.iter() {
        println!("Player HP: {}", player_stats.hit_points);
    }
}

#[derive(Event)]
pub struct PlayerRifleCoolDownChange {}

pub fn update_player_rifle_cooldown(
    mut commands: Commands,
    mut player_attack_speed_change_events: EventReader<PlayerRifleCoolDownChange>,
    rifle_query: Query<&mut Weapon, With<Rifle>>,
    mut timer_query: Query<Entity, (With<WeaponCoolDown>, With<Rifle>)>,
) {
    for _ in player_attack_speed_change_events.iter() {
        for entity in &mut timer_query {
            for rifle in rifle_query.iter() {
                println!("new rifle cooldown value {}", rifle.cooldown);
                let updated_rifle_timer = WeaponCoolDown {
                    timer: Timer::new(
                        Duration::from_millis(rifle.cooldown as u64),
                        TimerMode::Repeating,
                    ),
                };
                commands.entity(entity).remove::<WeaponCoolDown>();
                commands.entity(entity).insert(updated_rifle_timer);
            }
        }
    }
}

#[derive(Event)]
pub struct PlayerShotGunCoolDownChange {}

pub fn update_player_shotgun_cooldown(
    mut commands: Commands,
    mut player_shotgun_cooldown_change_events: EventReader<PlayerShotGunCoolDownChange>,
    shotgun_query: Query<&Weapon, With<Weapon>>,
    mut timer_query: Query<Entity, (With<WeaponCoolDown>, With<Weapon>)>,
) {
    for _ in player_shotgun_cooldown_change_events.iter() {
        for entity in &mut timer_query {
            for shotgun in shotgun_query.iter() {
                println!("new shotgun cooldown value {}", shotgun.cooldown);
                let updated_attack_timer = WeaponCoolDown {
                    timer: Timer::new(
                        Duration::from_millis(shotgun.cooldown as u64),
                        TimerMode::Repeating,
                    ),
                };
                commands.entity(entity).remove::<WeaponCoolDown>();
                commands.entity(entity).insert(updated_attack_timer);
            }
        }
    }
}
