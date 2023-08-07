use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Default, Resource)]
pub struct UiState {
    value: f32,
    is_pistol_equiped: bool,
    is_shotgun_equiped: bool,
}

pub fn ui_example_system(mut ui_state: ResMut<UiState>, mut contexts: EguiContexts) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx, |ui| {
            ui.heading("Debug Panel");

            ui.add(
                egui::Slider::new(&mut ui_state.value, 110.0..=3000.0).text("Player attack speed"),
            );
            if ui.button("+").clicked() {
                ui_state.value += 100.0;
            }
            if ui.button("-").clicked() {
                ui_state.value -= 100.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_pistol_equiped, "Pistol");

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            ui.checkbox(&mut ui_state.is_shotgun_equiped, "Shotgun");
        });
}
