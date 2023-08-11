use std::time::Duration;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    global::Stats,
    player::{Player, PlayerRifleCoolDownChange, PlayerShotGunCoolDownChange},
    weapon::{
        rifle::{Rifle, RifleBundle, RifleCoolDown},
        shotgun::{ShotGunCoolDown, Shotgun, ShotgunBundle},
    },
};

#[derive(Default, Resource)]
pub struct UiState {
    player_movement_speed: f32,
    is_shotgun_equiped: bool,
    player_shotgun_cooldown: f32,
    player_shotgun_bullet_count: f32,
    is_rifle_equiped: bool,
    player_rifle_cooldown: f32,
}

pub fn initialize_uistate(mut ui_state: ResMut<UiState>) {
    ui_state.player_movement_speed = 400.;
    ui_state.is_rifle_equiped = true;
    ui_state.player_rifle_cooldown = 10.;
    ui_state.is_shotgun_equiped = true;
    ui_state.player_shotgun_cooldown = 10.;
    ui_state.player_shotgun_bullet_count = 8.;
}

pub fn ui_example_system(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut shotgun_cooldown_event_writer: EventWriter<PlayerShotGunCoolDownChange>,
    mut rifle_cooldown_event_writer: EventWriter<PlayerRifleCoolDownChange>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            /*ui.horizontal(|ui| {
                ui.heading("Player Stats: ");
            });

             ui.label("Movement Speed");
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Slider::new(&mut ui_state.player_movement_speed, 10.0..=3000.0)
                            .logarithmic(true),
                    )
                    .changed()
                {
                    shotgun_cooldown_event_writer.send(PlayerShotGunCoolDownChange {});
                }
                if ui.button("-50ms").clicked() {
                    ui_state.player_movement_speed -= 50.;
                    shotgun_cooldown_event_writer.send(PlayerShotGunCoolDownChange {});
                }
                if ui.button("+50ms").clicked() {
                    ui_state.player_movement_speed += 50.;
                    shotgun_cooldown_event_writer.send(PlayerShotGunCoolDownChange {});
                }
            });*/

            ui.allocate_space(egui::Vec2::new(1.0, 30.0));
            ui.horizontal(|ui| {
                ui.heading("Player Weapons: ");
            });

            ui.checkbox(&mut ui_state.is_shotgun_equiped, "Shotgun");
            let bullet_label = format!(
                "{}{}",
                "shotgun bullets: ", ui_state.player_shotgun_bullet_count
            );
            ui.label("Cooldown");
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Slider::new(&mut ui_state.player_shotgun_cooldown, 10.0..=3000.0)
                            .logarithmic(true),
                    )
                    .changed()
                {
                    shotgun_cooldown_event_writer.send(PlayerShotGunCoolDownChange {});
                }
                if ui.button("-100ms").clicked() {
                    ui_state.player_shotgun_cooldown -= 100.;
                    shotgun_cooldown_event_writer.send(PlayerShotGunCoolDownChange {});
                }
                if ui.button("+100ms").clicked() {
                    ui_state.player_shotgun_cooldown += 100.;
                    shotgun_cooldown_event_writer.send(PlayerShotGunCoolDownChange {});
                }
            });

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.horizontal(|ui| {
                ui.label(bullet_label);
                if ui.button("+1 Bullet").clicked() {
                    ui_state.player_shotgun_bullet_count += 1.;
                }
                if ui.button("-1 Bulllet").clicked() {
                    ui_state.player_shotgun_bullet_count -= 1.;
                }
            });

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_rifle_equiped, "Rifle");
            ui.label("Cooldown");
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Slider::new(&mut ui_state.player_rifle_cooldown, 10.0..=3000.0)
                            .logarithmic(true),
                    )
                    .changed()
                {
                    rifle_cooldown_event_writer.send(PlayerRifleCoolDownChange {});
                }
                if ui.button("r-100ms").clicked() {
                    ui_state.player_rifle_cooldown -= 100.;
                    rifle_cooldown_event_writer.send(PlayerRifleCoolDownChange {});
                }
                if ui.button("r+100ms").clicked() {
                    ui_state.player_rifle_cooldown += 100.;
                    rifle_cooldown_event_writer.send(PlayerRifleCoolDownChange {});
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
                        fire_rate: ui_state.player_rifle_cooldown as f64,
                        ..default()
                    },
                    cooldown: RifleCoolDown {
                        timer: Timer::new(
                            Duration::from_millis(ui_state.player_rifle_cooldown as u64),
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
                        bullet_count: ui_state.player_shotgun_bullet_count as i32,
                        fire_rate: ui_state.player_shotgun_cooldown as f64,
                        ..default()
                    },
                    cooldown: ShotGunCoolDown {
                        timer: Timer::new(
                            Duration::from_millis(ui_state.player_shotgun_cooldown as u64),
                            TimerMode::Repeating,
                        ),
                    },
                });
            }
        }
    } else if !player_shotgun_entity_query.is_empty() {
        for player_entity in &mut player_entity_query {
            println!("Removing Shotgun!");
            commands
                .entity(player_entity)
                .remove::<Shotgun>()
                .remove::<ShotGunCoolDown>();
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
        player_rifle.fire_rate = ui_state.player_rifle_cooldown as f64;
    }
}

pub fn update_player_shotgun_stats(
    mut player_shotgun_query: Query<&mut Shotgun, With<Player>>,
    ui_state: ResMut<UiState>,
) {
    for mut player_shotgun in &mut player_shotgun_query {
        player_shotgun.fire_rate = ui_state.player_shotgun_cooldown as f64;
        player_shotgun.bullet_count = ui_state.player_shotgun_bullet_count as i32;
    }
}
