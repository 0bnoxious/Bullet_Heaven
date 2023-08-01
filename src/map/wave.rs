use std::time::Duration;

use bevy::prelude::*;

use crate::{global::MobType, mob::infected::Infected};

pub const NUMBER_OF_WAVES: i32 = 10;
pub const TIME_BETWEEN_WAVES: u64 = 1;
pub const DEFAULT_MAX_WAVE_MOB_COUNT: u64 = 200;
pub const DEFAULT_DELAY_BETWEEN_SPAWN: u64 = 200;

#[derive(Component)]
pub struct WaveTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Wave {
    wave_number: i32,
    max_mob_count: u64,
    mobs_types: Vec<MobType>,
    kill_count: i32,
    spawn_delay: Timer,
}

pub fn spawn_waves_manager(mut commands: Commands) {
    let mut mob_type_vec: Vec<MobType> = Vec::new();
    mob_type_vec.push(MobType::Infected);

    commands.spawn((
        WaveTimer {
            timer: Timer::new(
                Duration::from_millis(TIME_BETWEEN_WAVES),
                TimerMode::Repeating,
            ),
        },
        Wave {
            wave_number: 1,
            max_mob_count: DEFAULT_MAX_WAVE_MOB_COUNT,
            mobs_types: mob_type_vec,
            kill_count: 0,
            spawn_delay: Timer::new(
                Duration::from_millis(DEFAULT_DELAY_BETWEEN_SPAWN),
                TimerMode::Repeating,
            ),
        },
    ));
}

pub fn manage_waves(
    mut commands: Commands,
    mut wave_time_query: Query<&WaveTimer>,
    mut wave_query: Query<&Wave>,
) {
}
