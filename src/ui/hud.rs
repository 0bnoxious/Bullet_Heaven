use std::time::Duration;

use bevy::prelude::*;
use kayak_ui::{
    prelude::{widgets::*, *},
    CameraUIKayak,
};

use crate::ui::hud::wave_timer::{
    hud_wave_timer_render, HudWaveTimerBundle, HudWaveTimerUpdate, HudWaveTimerWidget,
};

pub mod wave_timer;

pub fn setup_hud(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    let camera_entity = commands
        .spawn((Camera2dBundle::default(), CameraUIKayak))
        .id();
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_system(
        HudWaveTimerWidget::default().get_name(),
        widget_update::<HudWaveTimerWidget, EmptyState>,
        hud_wave_timer_render,
    );
    widget_context.add_widget_data::<HudWaveTimerWidget, EmptyState>();

    let parent_id = None;
    commands.spawn(HudWaveTimerUpdate {
        timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
    });
    rsx! {
        <KayakAppBundle>
           <HudWaveTimerBundle/>
        </KayakAppBundle>
    };
    commands.spawn((widget_context, EventDispatcher::default()));
}
