use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::system::SystemParam;
use bevy::input::gamepad::GamepadButton;
use bevy::prelude::*;
use bevy::sprite::SpriteBundle;
use bevy::transform::commands;
use rand::Rng;
use rand::rngs::ThreadRng;
use std::time::Duration;

pub const BOXSIZE: f32 = 720.;

pub const PERSONCOUNT: i32 = 10;
pub const PERSONSPEED: f32 = 50.;
pub const PERSONSIZE: f32 = 10.;

pub const PLAYERSPEED: f32 = 100.;
pub const ATTACKSPEED: u64 = 1;
pub const PROJECTILESPEED: f32 = 10.;
pub const PROJECTILELIFESPAN: u64 = 1;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins/*, LogDiagnosticsPlugin::default(), FrameTimeDiagnosticsPlugin::default()*/))
        .add_systems(Startup, (setup, spawn_player, populate))
        .add_systems(
            Update,
            (
                move_population,
                move_projectile,
                update_population_direction,
                infect,
                define_space,
                gamepad_input,
                player_attack,
                dispawn_projectile,
            ),
        )
        .run()
}

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(TimerRes {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
    });
}

#[derive(Resource)]
struct TimerRes {
    timer: Timer,
}

#[derive(Component)]
struct InfectTimer {
    timer: Timer,
}

#[derive(Component)]
struct AttackTimer {
    timer: Timer,
}

#[derive(Component)]
struct ProjectileTimer {
    timer: Timer,
}

#[derive(Component)]
pub struct Person {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Projectile {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Player {
    pub is_infected: bool,
    pub direction: Vec3,
}

#[derive(Component)]
struct Infected;

fn define_space(mut query: Query<&mut Transform, With<Person>>) {
    let minxy = (-BOXSIZE / 2.) - PERSONSIZE / 2.;
    let maxxy = (BOXSIZE / 2.) - PERSONSIZE / 2.;

    for mut transform in query.iter_mut() {
        let mut translation = transform.translation;

        if translation.x < minxy {
            translation.x = minxy;
        } else if translation.x > maxxy {
            translation.x = maxxy
        }
        if translation.y < minxy {
            translation.y = minxy;
        } else if translation.y > maxxy {
            translation.y = maxxy
        }

        transform.translation = translation
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: (Some(Vec2 {
                    x: PERSONSIZE,
                    y: PERSONSIZE,
                })),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        InfectTimer {
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
        },
        AttackTimer {
            timer: Timer::new(Duration::from_millis(ATTACKSPEED), TimerMode::Repeating),
        },
        Player{
            is_infected: false,
            direction: Vec3::ZERO,
        }
    ));
}

fn populate(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    //patient 0
    commands.spawn((
        Person {
            direction: generate_velocity(&mut rng),
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: (Some(Vec2 {
                    x: PERSONSIZE,
                    y: PERSONSIZE,
                })),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        InfectTimer {
            timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
        },
        Infected,
    ));

    let mut v = vec![];
    for _ in 0..PERSONCOUNT {
        let posx = rng.gen_range(-BOXSIZE..=BOXSIZE);
        let posy = rng.gen_range(-BOXSIZE..=BOXSIZE);

        v.push((
            Person {
                direction: generate_velocity(&mut rng),
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: (Some(Vec2 {
                        x: PERSONSIZE,
                        y: PERSONSIZE,
                    })),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(posx, posy, 0.)),
                ..default()
            },
            InfectTimer {
                timer: Timer::new(Duration::from_millis(200), TimerMode::Repeating),
            },
        ));
    }
    commands.spawn_batch(v);
}

fn player_attack(
    time: Res<Time>, 
    mut attack_timer_query: Query<&mut AttackTimer>,
    mut player_counter: PlayerProjectileSpawner,
) {
    let mut attack_timer = attack_timer_query.get_single_mut().unwrap();
    attack_timer.timer.tick(time.delta());
    if attack_timer.timer.finished() {
        player_counter.spawn_projectile();
    }
}

#[derive(SystemParam)]
struct PlayerProjectileSpawner<'w, 's> {
    commands: Commands<'w, 's>,
    players: Query<'w, 's, &'static Transform, With<Player>>,
}

impl<'w, 's> PlayerProjectileSpawner<'w, 's> {
    fn spawn_projectile(&mut self) {
        let player_position = self.players.single().translation;
    
        self.commands.spawn((
            Projectile {
                direction: Vec3::ZERO,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: (Some(Vec2 {
                        x: PERSONSIZE,
                        y: PERSONSIZE,
                    })),
                    ..default()
                },
                transform: Transform::from_translation(player_position),
                ..default()
            },
            ProjectileTimer {
                timer: Timer::new(Duration::from_secs(PROJECTILELIFESPAN), TimerMode::Once),
            }
        ));
    }
}

enum AimingStyle {
    Random,
    Closest,
    HomingClosest,
    Mouse,
    HomingMouse,
    Direction,
}

fn move_projectile(
    mut projectile_query: Query<(&mut Transform, &Projectile)>,
    infected_query: Query<&Transform, (With<Infected>, With<Person>, Without<Projectile>)>,
    time: Res<Time>, 
) {
    let aim_type = AimingStyle::Random;

    match aim_type {
        AimingStyle::Random => {
            let mut rng = rand::thread_rng();
            let velocity = generate_velocity(&mut rng);
            for (mut transform, projectile) in &mut projectile_query {
                if transform.translation == Vec3::ZERO {
                    
                    transform.translation += velocity * PROJECTILESPEED * time.delta_seconds();
                } else {
                    
                    //transform.translation += velocity * PROJECTILESPEED * time.delta_seconds();
                    let direction = transform.translation * PROJECTILESPEED * time.delta_seconds();
                    transform.translation += direction;
                }

            }
        },
        AimingStyle::HomingClosest => {
            let mut closest_distance = 1000.;
            let mut closest_infected_translation = Vec3::ZERO;

            for (mut projectile_transform, _) in &mut projectile_query {

                let projectile_translation = projectile_transform.translation;

                for infected_transform in &mut infected_query.iter() {

                    let infected_translation = infected_transform.translation;

                    let distance = Vec3::distance(projectile_translation, infected_translation);

                    if distance < closest_distance{

                        closest_distance = distance;
                        closest_infected_translation = infected_translation;  
                    }
                }

                 // get the vector from the projectile to the closest infected and normalize it.
                let to_closest = closest_infected_translation - projectile_translation;

                // get the quaternion to rotate from the initial projectile facing direction to the direction
                // facing the closest infected
                let rotate_to_infected = Quat::from_rotation_arc(Vec3::Y, to_closest);

                // rotate the projectile to face the closest infected
                projectile_transform.rotation = rotate_to_infected;
                projectile_transform.translation += to_closest.normalize() * PROJECTILESPEED * time.delta_seconds();

            }
        },
        AimingStyle::Direction => println!("Direction!"),
        AimingStyle::Mouse => println!("Mouse!"),
        AimingStyle::Closest => println!("HomingClosest!"),
        AimingStyle::HomingMouse => println!("HomingMouse!"),
    }
}

fn dispawn_projectile(time: Res<Time>, 
    mut projectile_query: Query<(Entity, &mut ProjectileTimer)>,
    mut commands: Commands
    ){
    for (projectile_entity, mut projectile_timer) in projectile_query.iter_mut() {
        projectile_timer.timer.tick(time.delta());
        if projectile_timer.timer.just_finished() {
            commands.entity(projectile_entity).despawn_recursive();
        }
    }
}

fn update_population_direction(
    mut query: Query<&mut Person>,
    time: Res<Time>,
    mut timer_res: ResMut<TimerRes>,
) {
    timer_res.timer.tick(time.delta());

    let mut rng = rand::thread_rng();
    for mut person in &mut query {
        if timer_res.timer.just_finished() {
            person.direction = generate_velocity(&mut rng);
        }
    }
}

fn move_population(mut query: Query<(&mut Transform, &Person)>, time: Res<Time>) {
    for (mut transform, person) in &mut query {
        transform.translation += person.direction * PERSONSPEED * time.delta_seconds();
    }
}

#[allow(clippy::type_complexity)]
fn infect(
    mut commands: Commands,
    query_infected: Query<&Transform, With<Infected>>,
    mut query_healthy: Query<(Entity, &Transform, &mut Sprite, &mut InfectTimer), (With<Person>, Without<Infected>)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for infected_transform in &query_infected {
        for (entity, healthy_transform, mut sprite,mut infect_timer) in &mut query_healthy {
            let distance = infected_transform.translation.distance(healthy_transform.translation);
            if distance < PERSONSIZE {
                //attempt to infect once every 1/5 second
                infect_timer.timer.tick(time.delta());
                if infect_timer.timer.finished() {
                    // 1/5 chance to infect
                    let infect = rng.gen_range(0..=4);
                    if infect == 4 {
                        sprite.color = Color::RED;
                        commands.entity(entity).insert(Infected);
                    }
                }
            }
        }
    }
}

fn generate_velocity(rng: &mut ThreadRng) -> Vec3 {
    let velx = rng.gen_range(-1.0..1.0);
    let vely = rng.gen_range(-1.0..1.0);

    Vec3::new(velx, vely, 0.)
}

// TODO: leafwing-input-manager
fn gamepad_input(
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<&mut Transform, With<Player>>,
    gamepads: Res<Gamepads>,
    time: Res<Time>,
) {
    let Some(gamepad) = gamepads.iter().next() else { 
        return; 
    }; 

    // In a real game, the buttons would be configurable, but here we hardcode them
    let up_dpad = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadUp,
    };
    let down_dpad = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadDown,
    };
    let left_dpad = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadLeft,
    };
    let right_dpad = GamepadButton {
        gamepad,
        button_type: GamepadButtonType::DPadRight,
    };

    if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
        info!("{:?} just pressed South", gamepad);
    }

    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if buttons.pressed(up_dpad) {
            direction += Vec3::new(0., 1., 0.)
        }

        if buttons.pressed(down_dpad) {
            direction += Vec3::new(0., -1., 0.)
        }

        if buttons.pressed(left_dpad) {
            direction += Vec3::new(-1., 0., 0.)
        }

        if buttons.pressed(right_dpad) {
            direction += Vec3::new(1., 0., 0.)
        }

        transform.translation += direction * PLAYERSPEED * time.delta_seconds();
    }
}
