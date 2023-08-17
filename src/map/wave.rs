use std::time::Duration;

use bevy::prelude::*;

use crate::{
    global::MobType,
    mob::{infected::Infected, spawner::MobSpawner},
};

pub const NUMBER_OF_WAVES: u32 = 10;
pub const MAX_WAVE_MOB_COUNT: u32 = 1;
pub const TIME_BETWEEN_WAVES: u32 = 1000;
pub const DELAY_BETWEEN_SPAWN: u32 = 100;

#[derive(Component, Clone)]
pub struct WaveManager {
    pub wave_timer: Timer,
    pub spawn_timer: Timer,
    pub waves: Vec<Wave>,
    pub current_wave_number: u32,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Wave {
    //wave_number: i32,
    pub max_mob_count: u32,
    pub mobs_type: MobType,
    //kill_count: i32,
}

#[derive(Event)]
pub struct WaveTimerChange {
    pub new_wave_cooldown: u32,
}

#[derive(Event)]
pub struct WaveEnemyCountChange {
    pub new_enemy_count: u32,
}

pub fn spawn_waves_manager(mut commands: Commands) {
    commands.spawn(WaveManager {
        wave_timer: Timer::new(
            Duration::from_millis(TIME_BETWEEN_WAVES as u64),
            TimerMode::Repeating,
        ),
        spawn_timer: Timer::new(
            Duration::from_millis(DELAY_BETWEEN_SPAWN as u64),
            TimerMode::Repeating,
        ),
        waves: build_waves(),
        current_wave_number: 0,
    });
}

pub fn manage_waves(
    mut wave_manager_query: Query<&mut WaveManager>,
    mut mob_spawner: MobSpawner,
    infected_query: Query<&Infected>,
    time: Res<Time>,
) {
    if !wave_manager_query.is_empty() {
        let mut wave_manager = wave_manager_query.single_mut();

        wave_manager.wave_timer.tick(time.delta());
        if wave_manager.wave_timer.just_finished() {
            wave_manager.current_wave_number += 1;
            if wave_manager.current_wave_number >= NUMBER_OF_WAVES {
                wave_manager.current_wave_number = 0;
            }
        }

        wave_manager.spawn_timer.tick(time.delta());
        if wave_manager.spawn_timer.just_finished() {
            let max_enemies =
                wave_manager.waves[wave_manager.current_wave_number as usize].max_mob_count as i32;
            let missing_mobs = max_enemies - infected_query.iter().count() as i32;

            if missing_mobs > 0 {
                mob_spawner.spawn_mob(
                    wave_manager.waves[wave_manager.current_wave_number as usize].mobs_type,
                    missing_mobs as u64,
                );
            }
        }
    }
}

pub fn build_waves() -> Vec<Wave> {
    let mut waves: Vec<Wave> = Vec::new();

    for (wave_num, _) in (0..NUMBER_OF_WAVES).enumerate() {
        let mob_type;
        if wave_num % 2 != 0 {
            mob_type = MobType::InfectedRanged;
        } else {
            mob_type = MobType::Infected;
        }

        let wave = Wave {
            max_mob_count: MAX_WAVE_MOB_COUNT,
            mobs_type: mob_type,
            //kill_count: 0,
        };
        waves.push(wave);
    }

    waves
}
