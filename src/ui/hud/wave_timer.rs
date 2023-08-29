use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::*};

#[derive(Bundle)]
pub struct HudWaveTimerBundle {
    pub props: HudWaveTimerWidget,
    pub widget_name: WidgetName,
}

impl Default for HudWaveTimerBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            widget_name: HudWaveTimerWidget::default().get_name(),
        }
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct HudWaveTimerWidget {
    pub wave_time: u32,
}

#[derive(Component)]
pub struct HudWaveTimerUpdate {
    pub timer: Timer,
}

impl Widget for HudWaveTimerWidget {}

pub fn hud_wave_timer_render(
    In(entity): In<Entity>,
    widget_context: ResMut<KayakWidgetContext>,
    mut commands: Commands,
    query: Query<&HudWaveTimerWidget>,
) -> bool {
    // HudWaveTimerWidget is updated twice, idk why :(
    // this boolean is used to render the timer only once
    let mut rendered = false;
    query.iter().for_each(|widget| {
        if !rendered {
            let parent_id = Some(entity);
            let mut secs = widget.wave_time;
            let mins = secs / 60;
            secs %= 60;
            let string = format!("{mins:0>2}:{secs:0>2}");
            rsx! {
                <TextWidgetBundle
                    text={TextProps {
                        content: string,
                        size: 20.0,
                        alignment: Alignment::Middle,
                        ..Default::default()
                    }}
                />
            };

            rendered = true;
        }
    });

    true
}

pub fn update_hud_wave_timer_value(
    mut hud_wave_timer_update_query: Query<&mut HudWaveTimerUpdate>,
    mut hud_wave_timer_props_query: Query<&mut HudWaveTimerWidget, Without<PreviousWidget>>,
    time_res: Res<Time>,
) {
    for mut wave_timer_update in &mut hud_wave_timer_update_query {
        wave_timer_update.timer.tick(time_res.delta());
        if wave_timer_update.timer.finished() {
            for mut wave_timer_props in &mut hud_wave_timer_props_query {
                wave_timer_props.wave_time += 1;
            }
        }
    }
}
