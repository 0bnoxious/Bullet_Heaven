use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    global::Stats,
    map::wave::{WaveEnemyCountChange, WaveManager, WaveTimerChange},
    player::{Player, PlayerRifleCoolDownChange, PlayerShotGunCoolDownChange},
    weapon::{
        rifle::{Rifle, RifleBundle, RifleCoolDown},
        shotgun::{Shotgun, ShotgunBundle, ShotgunCoolDown, DEFAULT_SHOTGUN_AIM_TYPE},
    },
};

#[derive(Default, Resource)]
pub struct UiState {
    player_movement_speed: u32,
    is_rifle_equiped: bool,
    player_rifle_cooldown: u32,
    is_shotgun_equiped: bool,
    player_shotgun_cooldown: u32,
    player_shotgun_bullet_count: u32,
    player_shotgun_spread: u32,
    wave_timer_cooldown: u32,
    enemy_spawn_cooldown: u32,
    enemy_count: u32,
}

pub fn initialize_uistate(mut ui_state: ResMut<UiState>) {
    ui_state.player_movement_speed = 2;
    ui_state.is_rifle_equiped = true;
    ui_state.player_rifle_cooldown = 10;
    ui_state.is_shotgun_equiped = true;
    ui_state.player_shotgun_cooldown = 10;
    ui_state.player_shotgun_bullet_count = 8;
    ui_state.player_shotgun_spread = 15;
    ui_state.wave_timer_cooldown = 1000;
    ui_state.enemy_spawn_cooldown = 500;
    ui_state.enemy_count = 200;
}

pub fn ui_example_system(
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
                    ui_state.enemy_count += 1000;
                    enemy_count_event_writer.send(WaveEnemyCountChange {
                        new_enemy_count: ui_state.enemy_count,
                    });
                }
            });
        });
}

pub fn toggle_rifle(
    mut commands: Commands,
    player_rifle_entity_query: Query<Entity, (With<Rifle>, With<Player>)>,
    mut player_entity_query: Query<Entity, With<Player>>,
    ui_state: ResMut<UiState>,
) {
    if ui_state.is_rifle_equiped {
        if player_rifle_entity_query.is_empty() {
            for player_entity in &mut player_entity_query {
                println!("Adding Rifle!");
                commands.entity(player_entity).insert(RifleBundle {
                    rifle: Rifle {
                        cooldown: ui_state.player_shotgun_spread,
                        ..default()
                    },
                    cooldown: RifleCoolDown {
                        timer: Timer::new(
                            Duration::from_millis(ui_state.player_shotgun_spread as u64),
                            TimerMode::Repeating,
                        ),
                    },
                });
            }
        }
    } else if !player_rifle_entity_query.is_empty() {
        for player_entity in &mut player_entity_query {
            println!("Removing Rifle!");
            commands
                .entity(player_entity)
                .remove::<Rifle>()
                .remove::<RifleCoolDown>();
        }
    }
}

pub fn toggle_shotgun(
    mut commands: Commands,
    player_shotgun_entity_query: Query<Entity, (With<Shotgun>, With<Player>)>,
    mut player_entity_query: Query<Entity, With<Player>>,
    ui_state: ResMut<UiState>,
) {
    if ui_state.is_shotgun_equiped {
        if player_shotgun_entity_query.is_empty() {
            for player_entity in &mut player_entity_query {
                println!("Adding Shotgun!");
                commands.entity(player_entity).insert(ShotgunBundle {
                    shotgun: Shotgun {
                        bullet_count: ui_state.player_shotgun_bullet_count,
                        cooldown: ui_state.player_shotgun_cooldown,
                        ..default()
                    },
                    cooldown: ShotgunCoolDown {
                        timer: Timer::new(
                            Duration::from_millis(ui_state.player_shotgun_cooldown as u64),
                            TimerMode::Repeating,
                        ),
                    },
                    aim_type: DEFAULT_SHOTGUN_AIM_TYPE,
                });
            }
        }
    } else if !player_shotgun_entity_query.is_empty() {
        for player_entity in &mut player_entity_query {
            println!("Removing Shotgun!");
            commands
                .entity(player_entity)
                .remove::<Shotgun>()
                .remove::<ShotgunCoolDown>();
        }
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
    mut player_rifle_query: Query<&mut Rifle, With<Player>>,
    ui_state: ResMut<UiState>,
) {
    for mut player_rifle in &mut player_rifle_query {
        player_rifle.cooldown = ui_state.player_rifle_cooldown;
    }
}

pub fn update_player_shotgun_stats(
    mut player_shotgun_query: Query<&mut Shotgun, With<Player>>,
    ui_state: ResMut<UiState>,
) {
    for mut player_shotgun in &mut player_shotgun_query {
        player_shotgun.cooldown = ui_state.player_shotgun_cooldown;
        player_shotgun.bullet_count = ui_state.player_shotgun_bullet_count;
        player_shotgun.spread = ui_state.player_shotgun_spread;
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
