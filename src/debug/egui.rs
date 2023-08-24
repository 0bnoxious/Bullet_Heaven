use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_xpbd_2d::prelude::Position;

use crate::{
    global::{AimType, Stats},
    map::wave::{WaveEnemyCountChange, WaveManager, WaveTimerChange},
    player::Player,
    weapon::{rifle::Rifle, shotgun::Shotgun, Weapon, WeaponCoolDown},
};

use super::{PlayerRifleCoolDownChange, PlayerShotGunCoolDownChange};

#[derive(Default, Resource)]
pub struct UiState {
    player_movement_speed: u32,
    is_rifle_equiped: bool,
    player_rifle_cooldown: u32,
    is_shotgun_equiped: bool,
    player_shotgun_cooldown: u32,
    player_shotgun_bullet_count: i32,
    player_shotgun_spread: u32,
    wave_timer_cooldown: u32,
    enemy_spawn_cooldown: u32,
    enemy_count: u32,
}

pub fn initialize_uistate(mut ui_state: ResMut<UiState>) {
    ui_state.player_movement_speed = 2;
    ui_state.is_rifle_equiped = false;
    ui_state.player_rifle_cooldown = 10;
    ui_state.is_shotgun_equiped = false;
    ui_state.player_shotgun_cooldown = 10;
    ui_state.player_shotgun_bullet_count = 8;
    ui_state.player_shotgun_spread = 15;
    ui_state.wave_timer_cooldown = 1000;
    ui_state.enemy_spawn_cooldown = 500;
    ui_state.enemy_count = 1;
}

pub fn debug_ui_system(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut rifle_cooldown_event_writer: EventWriter<PlayerRifleCoolDownChange>,
    mut shotgun_cooldown_event_writer: EventWriter<PlayerShotGunCoolDownChange>,
    mut wave_timer_event_writer: EventWriter<WaveTimerChange>,
    mut enemy_count_event_writer: EventWriter<WaveEnemyCountChange>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Player Stats: ");
            });

            // Player Movement Speed
            ui.label("Movement Speed");
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(
                    &mut ui_state.player_movement_speed,
                    1..=10,
                ));
                if ui.button("-1").clicked() {
                    if ui_state.player_movement_speed <= 1 {
                        ui_state.player_movement_speed = 1;
                    } else {
                        ui_state.player_movement_speed -= 1;
                    }
                }
                if ui.button("+1").clicked() {
                    ui_state.player_movement_speed += 1;
                }
            });

            ui.allocate_space(egui::Vec2::new(1.0, 30.0));
            ui.horizontal(|ui| {
                ui.heading("Player Weapons: ");
            });

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_rifle_equiped, "Rifle");
            // Rifle Cooldown
            ui.label("Cooldown");
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Slider::new(&mut ui_state.player_rifle_cooldown, 10..=3000)
                            .logarithmic(true),
                    )
                    .changed()
                {
                    rifle_cooldown_event_writer.send(PlayerRifleCoolDownChange {});
                }
                if ui.button("r-10ms").clicked() {
                    let temp_cooldown = ui_state.player_rifle_cooldown as i32 - 10;
                    if temp_cooldown < 0 {
                        ui_state.player_rifle_cooldown = 10;
                    } else {
                        ui_state.player_rifle_cooldown = temp_cooldown as u32;
                    }
                    rifle_cooldown_event_writer.send(PlayerRifleCoolDownChange {});
                }
                if ui.button("r+10ms").clicked() {
                    ui_state.player_rifle_cooldown += 10;
                    rifle_cooldown_event_writer.send(PlayerRifleCoolDownChange {});
                }
            });

            ui.allocate_space(egui::Vec2::new(1.0, 20.0));
            ui.checkbox(&mut ui_state.is_shotgun_equiped, "Shotgun");
            // Shotgun Cooldown
            ui.label("Cooldown");
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Slider::new(&mut ui_state.player_shotgun_cooldown, 10..=3000)
                            .logarithmic(true),
                    )
                    .changed()
                {
                    shotgun_cooldown_event_writer.send(PlayerShotGunCoolDownChange {});
                }
                if ui.button("-10ms").clicked() {
                    let temp_cooldown = ui_state.player_shotgun_cooldown as i32 - 10;
                    if temp_cooldown < 0 {
                        ui_state.player_shotgun_cooldown = 10;
                    } else {
                        ui_state.player_shotgun_cooldown = temp_cooldown as u32;
                    }

                    shotgun_cooldown_event_writer.send(PlayerShotGunCoolDownChange {});
                }
                if ui.button("+10ms").clicked() {
                    ui_state.player_shotgun_cooldown += 10;
                    shotgun_cooldown_event_writer.send(PlayerShotGunCoolDownChange {});
                }
            });
            // Shotgun Bullets
            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.horizontal(|ui| {
                ui.label(format!(
                    "{}{}",
                    "Shotgun bullets: ", ui_state.player_shotgun_bullet_count
                ));
                if ui.button("+1 Bullet").clicked() {
                    ui_state.player_shotgun_bullet_count += 1;
                }
                if ui.button("-1 Bulllet").clicked() {
                    ui_state.player_shotgun_bullet_count -= 1;
                }
            });
            // Shotgun Spread
            ui.allocate_space(egui::Vec2::new(1.0, 5.0));
            ui.label("Spread");
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(
                    &mut ui_state.player_shotgun_spread,
                    1..=90,
                ));
                if ui.button("-3°").clicked() {
                    let temp_spread = ui_state.player_shotgun_spread as i32 - 3;
                    if temp_spread < 0 {
                        ui_state.player_shotgun_spread = 1;
                    } else {
                        ui_state.player_shotgun_spread = temp_spread as u32;
                    }
                }
                if ui.button("+3°").clicked() {
                    ui_state.player_shotgun_spread += 3;
                }
            });

            ui.allocate_space(egui::Vec2::new(1.0, 30.0));
            ui.horizontal(|ui| {
                ui.heading("Enemy Waves: ");
            });

            // Wave timer
            ui.label("Wave timer");
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Slider::new(&mut ui_state.wave_timer_cooldown, 100..=5000)
                            .logarithmic(true),
                    )
                    .changed()
                {
                    wave_timer_event_writer.send(WaveTimerChange {
                        new_wave_cooldown: ui_state.wave_timer_cooldown,
                    });
                }

                if ui.button("-1s").clicked() {
                    let temp_cooldown = ui_state.wave_timer_cooldown as i32 - 1000;
                    if temp_cooldown < 0 {
                        ui_state.wave_timer_cooldown = 10;
                    } else {
                        ui_state.wave_timer_cooldown = temp_cooldown as u32;
                    }

                    wave_timer_event_writer.send(WaveTimerChange {
                        new_wave_cooldown: ui_state.wave_timer_cooldown,
                    });
                }
                if ui.button("+1s").clicked() {
                    ui_state.wave_timer_cooldown += 1000;
                    wave_timer_event_writer.send(WaveTimerChange {
                        new_wave_cooldown: ui_state.wave_timer_cooldown,
                    });
                }
            });

            // Enemy Count
            ui.label("Enemy Count");
            ui.horizontal(|ui| {
                if ui
                    .add(egui::Slider::new(&mut ui_state.enemy_count, 1..=500).logarithmic(true))
                    .changed()
                {
                    enemy_count_event_writer.send(WaveEnemyCountChange {
                        new_enemy_count: ui_state.enemy_count,
                    });
                }

                if ui.button("-1e").clicked() {
                    let temp_enemies = ui_state.enemy_count as i32 - 1;
                    if temp_enemies < 0 {
                        ui_state.enemy_count = 10;
                    } else {
                        ui_state.enemy_count = temp_enemies as u32;
                    }

                    enemy_count_event_writer.send(WaveEnemyCountChange {
                        new_enemy_count: ui_state.enemy_count,
                    });
                }
                if ui.button("+1e").clicked() {
                    ui_state.enemy_count += 1;
                    enemy_count_event_writer.send(WaveEnemyCountChange {
                        new_enemy_count: ui_state.enemy_count,
                    });
                }
            });
        });
}

pub fn toggle_rifle(
    mut commands: Commands,
    mut rifle_entity_query: Query<Entity, (With<Rifle>, With<Weapon>)>,
    ui_state: ResMut<UiState>,
) {
    if ui_state.is_rifle_equiped {
        if rifle_entity_query.get_single_mut().is_err() {
            println!("Adding Rifle!");
            let weapon = Weapon {
                aim_type: AimType::Random,
                damage: 1,
                cooldown: ui_state.player_rifle_cooldown,
                spread: 1,
                bullet_count: 1,
            };
            commands.spawn((
                weapon,
                WeaponCoolDown {
                    timer: Timer::new(
                        Duration::from_millis(ui_state.player_rifle_cooldown as u64),
                        TimerMode::Repeating,
                    ),
                },
                Rifle,
                Position(Vec2::ZERO),
                Name::new("Rifle"),
            ));
        }
    } else if rifle_entity_query.get_single_mut().is_ok() {
        println!("Removing Rifle!");
        let rifle_entity = rifle_entity_query.get_single_mut().unwrap();
        commands.entity(rifle_entity).despawn_recursive();
    }
}

pub fn toggle_shotgun(
    mut commands: Commands,
    mut shotgun_entity_query: Query<Entity, (With<Weapon>, With<Shotgun>)>,
    ui_state: ResMut<UiState>,
) {
    if ui_state.is_shotgun_equiped {
        if shotgun_entity_query.get_single_mut().is_err() {
            println!("Adding Shotgun!");
            let weapon = Weapon {
                aim_type: AimType::Closest,
                damage: 1,
                cooldown: ui_state.player_shotgun_cooldown,
                spread: ui_state.player_shotgun_spread,
                bullet_count: ui_state.player_shotgun_bullet_count,
            };
            commands.spawn((
                weapon,
                WeaponCoolDown {
                    timer: Timer::new(
                        Duration::from_millis(ui_state.player_shotgun_cooldown as u64),
                        TimerMode::Repeating,
                    ),
                },
                Shotgun,
                Position(Vec2::ZERO),
                Name::new("Shotgun"),
            ));
        }
    } else if shotgun_entity_query.get_single_mut().is_ok() {
        println!("Removing Shotgun!");
        let shotgun_entity = shotgun_entity_query.get_single_mut().unwrap();
        commands.entity(shotgun_entity).despawn_recursive();
    }
}

pub fn update_player_stats(
    mut player_stats_query: Query<&mut Stats, With<Player>>,
    ui_state: ResMut<UiState>,
) {
    for mut player_stats in &mut player_stats_query {
        player_stats.movement_speed = ui_state.player_movement_speed;
    }
}

pub fn update_player_rifle_stats(
    mut rifle_query: Query<&mut Weapon, With<Rifle>>,
    ui_state: ResMut<UiState>,
) {
    for mut rifle in &mut rifle_query {
        rifle.cooldown = ui_state.player_rifle_cooldown;
    }
}

pub fn update_player_shotgun_stats(
    mut shotgun_query: Query<&mut Weapon, With<Shotgun>>,
    ui_state: ResMut<UiState>,
) {
    for mut shotgun in &mut shotgun_query {
        shotgun.cooldown = ui_state.player_shotgun_cooldown;
        shotgun.bullet_count = ui_state.player_shotgun_bullet_count;
        shotgun.spread = ui_state.player_shotgun_spread;
    }
}

pub fn update_wave_timer(
    mut wave_timer_change_events: EventReader<WaveTimerChange>,
    mut wave_manager_query: Query<&mut WaveManager>,
) {
    for event in wave_timer_change_events.iter() {
        for mut wave_manager in &mut wave_manager_query {
            wave_manager.wave_timer = Timer::new(
                Duration::from_millis(event.new_wave_cooldown as u64),
                TimerMode::Repeating,
            );
            println!("Changing wave timer to : {}", event.new_wave_cooldown);
        }
    }
}

pub fn update_enemy_count(
    mut enemy_count_change_even: EventReader<WaveEnemyCountChange>,
    mut wave_manager_query: Query<&mut WaveManager>,
) {
    for event in enemy_count_change_even.iter() {
        for mut wave_manager in &mut wave_manager_query {
            let mut wavenum = 0;
            for _ in 0..wave_manager.waves.len() - 1 {
                wavenum += 1;
                wave_manager.waves[wavenum].max_mob_count = event.new_enemy_count;
            }
            println!("Changing enemy count to : {}", event.new_enemy_count);
        }
    }
}
