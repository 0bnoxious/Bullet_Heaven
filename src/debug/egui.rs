use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    global::Stats,
    player::{Player, PlayerAttackSpeedChange},
    weapon::shotgun::Shotgun,
};

#[derive(Default, Resource)]
pub struct UiState {
    player_attack_speed: f32,
    player_shotgun_bullet_count: f32,
    is_pistol_equiped: bool,
    is_shotgun_equiped: bool,
}

pub fn initialize_uistate(mut ui_state: ResMut<UiState>) {
    ui_state.is_pistol_equiped = false;
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

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Debug Panel");

            ui.add(
                egui::Slider::new(&mut ui_state.player_attack_speed, 110.0..=3010.0)
                    .text("Player attack speed"),
            );

            if ui.button("Attack speed + 100ms").clicked() {
                ui_state.player_attack_speed += 100.;
                event_writer.send(PlayerAttackSpeedChange {});
            }
            if ui.button("Attack Speedd - 100ms").clicked() {
                ui_state.player_attack_speed -= 100.;
                event_writer.send(PlayerAttackSpeedChange {});
            }

            let bullet_label = format!(
                "{}{}",
                "shotgun bullets: ", ui_state.player_shotgun_bullet_count
            );
            ui.horizontal(|ui| {
                ui.label(bullet_label);
                if ui.button("Bulllet +1").clicked() {
                    ui_state.player_shotgun_bullet_count += 1.;
                }
                if ui.button("Bullet -1").clicked() {
                    ui_state.player_shotgun_bullet_count -= 1.;
                }
            });

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_pistol_equiped, "Pistol");

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_shotgun_equiped, "Shotgun");
        });
}

pub fn toggle_shotgun(
    mut commands: Commands,
    player_shotgun_entity_query: Query<Entity, (With<Shotgun>, With<Player>)>,
    mut player_entity_query: Query<Entity, With<Player>>,
    ui_state: ResMut<UiState>,
) {
    if ui_state.is_shotgun_equiped {
        if player_shotgun_entity_query.is_empty() {
            let playercount = player_entity_query.iter().count();
            for player_entity in &mut player_entity_query {
                println!("Adding shotgun to {} players!", playercount);
                commands
                    .entity(player_entity)
                    .insert(Shotgun { ..default() });
            }
        }
    } else if !player_shotgun_entity_query.is_empty() {
        for player_entity in &mut player_entity_query {
            println!("Removing shotgun!");
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
