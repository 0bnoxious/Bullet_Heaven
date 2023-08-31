use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::*};

use super::HudProps;

#[derive(Bundle)]
pub struct HudWaveTimerBundle {
    pub props: HudProps,
    pub widget_name: WidgetName,
    pub styles: KStyle,
}

impl Default for HudWaveTimerBundle {
    fn default() -> Self {
        Self {
            props: Default::default(),
            widget_name: HudProps::default().get_name(),
            styles: Default::default(),
        }
    }
}

#[derive(Component)]
pub struct HudWaveTimerUpdate {
    pub timer: Timer,
}

pub fn update_hud_wave_timer_value(
    mut hud_wave_timer_update_query: Query<&mut HudWaveTimerUpdate>,
    mut hud_wave_timer_props_query: Query<&mut HudProps, Without<PreviousWidget>>,
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

pub fn hud_wave_timer_render(
    In(entity): In<Entity>,
    widget_context: ResMut<KayakWidgetContext>,
    mut commands: Commands,
    query: Query<&HudProps>,
) -> bool {
    // HudWaveTimerWidget is updated twice, idk why :(
    // this boolean is used to render the timer only once
    let mut rendered = false;
    query.iter().for_each(|widget| {
        if !rendered {
            let test_style = KStyle {
                bottom: StyleProp::Value(Units::Stretch(0.0)),
                layout_type: StyleProp::Value(LayoutType::Column),
                top: StyleProp::Value(Units::Stretch(0.0)),
                height: StyleProp::Value(Units::Pixels(0.0)),
                width: StyleProp::Value(Units::Pixels(0.0)),
                ..Default::default()
            };

            let parent_id = Some(entity);
            let mut secs = widget.wave_time;
            let mins = secs / 60;
            secs %= 60;
            let string = format!("{mins:0>2}:{secs:0>2}");
            rsx! {
                <TextWidgetBundle
                    styles={test_style}
                    text={TextProps {
                        content: string,
                        size: 20.0,
                        alignment: Alignment::Middle,
                        ..Default::default()
                    }}
                >
                </TextWidgetBundle>
            };

            rendered = true;
        }
    });

    true
}
