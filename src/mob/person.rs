use std::time::Duration;

use bevy::prelude::*;

pub const PERSON_COUNT: i32 = 200;
pub const PERSON_SPEED: f32 = 20.;
pub const INFECTION_ATTEMPT_DELAY_MS: u64 = 200;

#[derive(Component)]
pub struct Person;

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
