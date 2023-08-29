use std::time::Duration;

use bevy::prelude::*;

use crate::ui::hud::wave_timer::HudWaveTimerUpdate;

pub mod wave_timer;

pub fn setup_hud(mut commands: Commands) {
    commands.spawn(HudWaveTimerUpdate {
        timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
    });
}
