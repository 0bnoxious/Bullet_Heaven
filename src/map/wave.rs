use std::time::Duration;

use bevy::prelude::*;

use crate::{
    global::MobType,
    mob::{infected::Infected, spawner::MobSpawner},
};

pub const NUMBER_OF_WAVES: usize = 10;
pub const MAX_WAVE_MOB_COUNT: u64 = 200;
pub const TIME_BETWEEN_WAVES: u64 = 1000;
pub const DELAY_BETWEEN_SPAWN: u64 = 100;

#[derive(Component)]
pub struct WaveManager {
    pub wave_timer: Timer,
    pub spawn_timer: Timer,
    pub waves: Vec<Wave>,
    pub current_wave_number: usize,
}

#[derive(Component, Debug, Clone)]
pub struct Wave {
    //wave_number: i32,
    max_mob_count: u64,
    mobs_types: Vec<MobType>,
    //kill_count: i32,
}

#[derive(Event)]
pub struct WaveTimerChange {
    pub new_wave_cooldown: u32,
}

/*pub fn wave_timer_change(
    mut wave_timer_change_events: EventReader<WaveTimerChange>,
    mut wave_manager_query: Query<&mut WaveManager>,
) {
    println!("ALLO");
    if !wave_timer_change_events.is_empty() {
        for event in wave_timer_change_events.iter() {
            for mut wave_manager in &mut wave_manager_query {
                wave_manager.wave_timer = Timer::new(
                    Duration::from_millis(event.cooldown as u64),
                    TimerMode::Repeating,
                )
            }
        }
    }
}*/

pub fn spawn_waves_manager(mut commands: Commands) {
    commands.spawn(WaveManager {
        wave_timer: Timer::new(
            Duration::from_millis(TIME_BETWEEN_WAVES),
            TimerMode::Repeating,
        ),
        spawn_timer: Timer::new(
            Duration::from_millis(DELAY_BETWEEN_SPAWN),
            TimerMode::Repeating,
        ),
        waves: build_waves(),
        current_wave_number: 0,
    });
}

pub fn manage_waves(
    //mut commands: Commands,
    mut wave_manager_query: Query<&mut WaveManager>,
    mut mob_spawner: MobSpawner,
    infected_query: Query<&Infected>,
    time: Res<Time>,
) {
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
        let missing_mobs = wave_manager.waves[wave_manager.current_wave_number].max_mob_count
            - infected_query.iter().count() as u64;

        mob_spawner.spawn_mob(
            wave_manager.waves[wave_manager.current_wave_number].mobs_types[0],
            missing_mobs,
        );
    }
}

pub fn build_waves() -> Vec<Wave> {
    let mut waves: Vec<Wave> = Vec::new();

    for (wave_num, _) in (0..NUMBER_OF_WAVES).enumerate() {
        let mut mob_types: Vec<MobType> = Vec::new();
        if wave_num % 2 != 0 {
            mob_types.push(MobType::InfectedRanged);
        } else {
            mob_types.push(MobType::Infected);
        }

        let wave = Wave {
            max_mob_count: MAX_WAVE_MOB_COUNT,
            mobs_types: mob_types,
            //kill_count: 0,
        };
        waves.push(wave);
    }

    waves
}
