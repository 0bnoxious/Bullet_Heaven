use std::time::Duration;

use bevy::ecs::system::SystemParam;
use bevy::input::gamepad::GamepadButton;
use bevy::prelude::*;

use crate::projectile::projectile::*;

pub const PLAYERSIZE: f32 = 10.;
pub const ATTACKSPEED: u64 = 10;
pub const PLAYERSPEED: f32 = 100.;

#[derive(Component)]
pub struct Player {
    pub direction: Vec3,
    pub aim_type: AimType,
}

#[derive(Component)]
pub struct AttackTimer {
    pub timer: Timer,
}

pub enum AimType {
    Random,
    Closest,
    HomingClosest,
    Mouse,
    HomingMouse,
    Direction,
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: (Some(Vec2 {
                    x: PLAYERSIZE,
                    y: PLAYERSIZE,
                })),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        AttackTimer {
            timer: Timer::new(Duration::from_millis(ATTACKSPEED), TimerMode::Repeating),
        },
        Player {
            direction: Vec3::ZERO,
            aim_type: AimType::Random,
        },
    ));
}

pub fn player_attack(
    time: Res<Time>,
    mut attack_timer_query: Query<&mut AttackTimer>,
    mut player_counter: PlayerProjectileSpawner,
) {
    let mut attack_timer = attack_timer_query.get_single_mut().unwrap();
    attack_timer.timer.tick(time.delta());
    if attack_timer.timer.finished() {
        player_counter.spawn_projectile();

        //let mut rng = rand::thread_rng();
        /*let sound_effect;
        // Randomly play one of the two sound effects.
        if rng.gen_bool(0.5) {
            sound_effect = "audio/tap.ogg";
        } else {
            sound_effect = "audio/ti.ogg";
        }*/
    }
}

#[derive(SystemParam)]
pub struct PlayerProjectileSpawner<'w, 's> {
    commands: Commands<'w, 's>,
    players: Query<'w, 's, &'static Transform, With<Player>>,
}

impl<'w, 's> PlayerProjectileSpawner<'w, 's> {
    pub fn spawn_projectile(&mut self) {
        let player_position = self.players.single().translation;

        self.commands.spawn((
            Projectile {
                direction: Vec3::ZERO,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::YELLOW,
                    custom_size: (Some(Vec2 {
                        x: PROJECTILESIZE,
                        y: PROJECTILESIZE,
                    })),
                    ..default()
                },
                transform: Transform::from_translation(player_position),
                ..default()
            },
            ProjectileTimer {
                timer: Timer::new(Duration::from_secs(PROJECTILELIFESPAN), TimerMode::Once),
            },
        ));
    }
}

// TODO: leafwing-input-manager
pub fn gamepad_input(
    buttons: Res<Input<GamepadButton>>,
    mut query: Query<&mut Transform, With<Player>>,
    gamepads: Res<Gamepads>,
    time: Res<Time>,
) {
    let Some(gamepad) = gamepads.iter().next() else {
        return;
    };

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
