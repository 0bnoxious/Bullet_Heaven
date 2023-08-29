use std::time::Duration;

use bevy::prelude::*;
use kayak_ui::prelude::*;

use crate::ui::hud::wave_timer::{HudWaveTimerBundle, HudWaveTimerUpdate};

pub mod wave_timer;

pub fn setup_hud(mut commands: Commands, widget_context: Res<KayakWidgetContext>) {
    println!("setting up player hud...");
    // widget_context.add_widget_system(
    //     HudWaveTimerWidget::default().get_name(),
    //     widget_update::<HudWaveTimerWidget, EmptyState>,
    //     hud_wave_timer_render,
    // );
    // widget_context.add_widget_data::<HudWaveTimerWidget, EmptyState>();

    let parent_id = None;
    commands.spawn(HudWaveTimerUpdate {
        timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
    });
    rsx! {
        <HudWaveTimerBundle/>
    };
    println!("player hud set!");
}
