use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    global::Stats,
    player::{Player, PlayerAttackSpeedChange},
    weapon::{rifle::Rifle, shotgun::Shotgun},
};

#[derive(Default, Resource)]
pub struct UiState {
    player_attack_speed: f32,
    player_shotgun_bullet_count: f32,
    is_rifle_equiped: bool,
    is_shotgun_equiped: bool,
}

pub fn initialize_uistate(mut ui_state: ResMut<UiState>) {
    ui_state.is_rifle_equiped = false;
    ui_state.is_shotgun_equiped = false;
    ui_state.player_attack_speed = 1010.;
    ui_state.player_shotgun_bullet_count = 1.;
}

pub fn ui_example_system(
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut event_writer: EventWriter<PlayerAttackSpeedChange>,
) {
    let ctx = contexts.ctx_mut();
    /*let attack_speed_slider =
    egui::Slider::new(&mut ui_state.player_attack_speed, 100.0..=3000.0).logarithmic(true);*/

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Player Stats: ");
            });

            ui.label("Attack Speed");
            ui.horizontal(|ui| {
                if ui
                    .add(
                        egui::Slider::new(&mut ui_state.player_attack_speed, 10.0..=3000.0)
                            .logarithmic(true),
                    )
                    .changed()
                {
                    event_writer.send(PlayerAttackSpeedChange {});
                }
                if ui.button("-100ms").clicked() {
                    ui_state.player_attack_speed -= 100.;
                    event_writer.send(PlayerAttackSpeedChange {});
                }
                if ui.button("+100ms").clicked() {
                    ui_state.player_attack_speed += 100.;
                    event_writer.send(PlayerAttackSpeedChange {});
                }
            });

            ui.allocate_space(egui::Vec2::new(1.0, 30.0));
            ui.horizontal(|ui| {
                ui.heading("Player Weapons: ");
            });

            ui.checkbox(&mut ui_state.is_shotgun_equiped, "Shotgun");
            let bullet_label = format!(
                "{}{}",
                "shotgun bullets: ", ui_state.player_shotgun_bullet_count
            );
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
                commands.entity(player_entity).insert(Rifle { ..default() });
            }
        }
    } else if !player_rifle_entity_query.is_empty() {
        for player_entity in &mut player_entity_query {
            println!("Removing Rifle!");
            commands.entity(player_entity).remove::<Rifle>();
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
                commands
                    .entity(player_entity)
                    .insert(Shotgun { ..default() });
            }
        }
    } else if !player_shotgun_entity_query.is_empty() {
        for player_entity in &mut player_entity_query {
            println!("Removing Shotgun!");
            commands.entity(player_entity).remove::<Shotgun>();
        }
    }
}

pub fn update_player_stats(
    mut player_stats_query: Query<&mut Stats, With<Player>>,
    ui_state: ResMut<UiState>,
) {
    for mut player_stats in &mut player_stats_query {
        player_stats.attack_speed = ui_state.player_attack_speed;
    }
}

pub fn update_player_shotgun(
    mut player_shotgun_query: Query<&mut Shotgun, With<Player>>,
    ui_state: ResMut<UiState>,
) {
    for mut player_shotgun in &mut player_shotgun_query {
        player_shotgun.bullet_count = ui_state.player_shotgun_bullet_count as i32;
    }
}
