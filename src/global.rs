use bevy::{
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use bevy_xpbd_2d::prelude::*;
use rand::{rngs::ThreadRng, Rng};

use crate::projectile::Damage;

pub const STARTING_GAME_STATE: GameState = GameState::Menu;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
}

#[derive(Component, Debug)]
pub struct Stats {
    pub hit_points: i32,
    pub movement_speed: u32,
    pub attack_speed: u32,
    pub defense: i32,
    pub damage: i32,
}

pub const DEFAULT_HP: i32 = 1;
pub const DEFAULT_DEFENSE: i32 = 0;
pub const DEFAULT_DAMAGE: i32 = 0;
pub const DEFAULT_ATTACK_SPEED: u32 = 0;
pub const DEFAULT_MOVEMENT_SPEED: u32 = 10;

impl Default for Stats {
    fn default() -> Self {
        Self {
            hit_points: DEFAULT_HP,
            movement_speed: DEFAULT_MOVEMENT_SPEED,
            attack_speed: DEFAULT_ATTACK_SPEED,
            defense: DEFAULT_DEFENSE,
            damage: DEFAULT_DAMAGE,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub enum AimType {
    Random,
    Closest,
    //Mouse,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum MobType {
    Infected,
    InfectedRanged,
    InfectedArmored,
    InfectedElite,
    InfectedCommander,
}

impl AimType {
    pub fn next(&self) -> Self {
        use AimType::*;
        match *self {
            Random => Closest,
            Closest => Random,
        }
    }
}

#[derive(PhysicsLayer)]
pub enum Layer {
    Default,
    Player,
    Person,
    PersonSensor,
    Infected,
    Projectile,
    Wall,
}

pub fn random_direction(rng: &mut ThreadRng) -> Vec3 {
    let x = rng.gen_range(-1.0..1.0);
    let y = rng.gen_range(-1.0..1.0);

    Vec3::new(x, y, 0.)
}

#[derive(Component)]
pub struct Dead;

pub fn resolve_damage(
    mut commands: Commands,
    mut damage_query: Query<(Entity, &mut Damage, &mut Stats)>,
) {
    for (entity, mut damage, mut stats) in &mut damage_query {
        let dmg_sum: i32 = damage.instances.iter().sum();
        stats.hit_points -= dmg_sum;
        damage.instances.clear();

        if stats.hit_points <= 0 {
            //println!("Entity : {entity:?} is dead!");
            commands.entity(entity).insert(Dead);
        }
    }
}

pub fn despawn_dead(mut query: Query<Entity, With<Dead>>, mut commands: Commands) {
    for entity in query.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn set_primary_window() -> WindowPlugin {
    WindowPlugin {
        primary_window: Some(Window {
            title: "Bullet Heaven".into(),
            resolution: (1920., 1080.).into(),
            present_mode: PresentMode::AutoVsync,
            // Tells wasm to resize the window according to the available canvas
            fit_canvas_to_parent: true,
            // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
            prevent_default_event_handling: false,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    }
}
