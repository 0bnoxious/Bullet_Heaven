use std::time::Duration;

use bevy::prelude::*;
use kayak_ui::prelude::*;

use crate::{
    global::{GameState, STARTING_GAME_STATE},
    ui::hud::wave_timer::HudWaveTimerBundle,
};

use self::wave_timer::HudWaveTimerUpdate;

pub mod wave_timer;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub enum Menu {
    Main,
    Settings,
}

#[derive(Component, Clone, PartialEq)]
pub struct HudProps {
    game_state: GameState,
    wave_time: u32,
    player_hp: u32,
}

impl Default for HudProps {
    fn default() -> Self {
        Self {
            game_state: STARTING_GAME_STATE,
            player_hp: Default::default(),
            wave_time: 0,
        }
    }
}

impl Widget for HudProps {}

#[derive(Bundle)]
pub struct HudBundle {
    pub props: HudProps,
    pub styles: KStyle,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for HudBundle {
    fn default() -> Self {
        Self {
            props: HudProps::default(),
            styles: KStyle::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: HudProps::default().get_name(),
        }
    }
}

pub fn setup_hud(mut commands: Commands) {
    commands.spawn(HudWaveTimerUpdate {
        timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
    });
}

pub fn hud_render(
    In(entity): In<Entity>,
    widget_context: ResMut<KayakWidgetContext>,
    mut commands: Commands,
) -> bool {
    let parent_id = Some(entity);
    rsx! {
        <HudWaveTimerBundle/>
    };

    // The boolean returned here tells kayak UI to update the tree. You can avoid tree updates by
    // returning false, but in practice this should be done rarely. As kayak diff's the tree and
    // will avoid tree updates if nothing has changed!
    true
}
