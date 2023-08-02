use bevy::prelude::*;
use std::time::Duration;

pub const HEALTHY_MOVEMENT_SPEED: f32 = 40.;
pub const INFECTION_ATTEMPT_DELAY_MS: u64 = 200;

#[derive(Component)]
pub struct Healthy;

#[derive(Component)]
pub struct InfectionAttemptTimer {
    pub timer: Timer,
}

impl Default for InfectionAttemptTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(
                Duration::from_millis(INFECTION_ATTEMPT_DELAY_MS),
                TimerMode::Repeating,
            ),
        }
    }
}
