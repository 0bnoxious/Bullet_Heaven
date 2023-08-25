use std::time::Duration;

use bevy::prelude::*;
use kayak_ui::{
    prelude::{widgets::*, *},
    CameraUIKayak,
};

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
    timer: Timer,
}

pub fn hud_wave_timer_render(
    In(entity): In<Entity>,
    mut widget_context: ResMut<KayakWidgetContext>,
    mut commands: Commands,
    query: Query<&HudWaveTimerWidget>,
) -> bool {
    if let Ok(my_widget) = query.get(entity) {
        query.iter().for_each(|widget| {
            let parent_id = Some(entity);
            rsx! {
                <TextWidgetBundle
                    text={TextProps {
                        content: widget.wave_time.to_string(),
                        size: 20.0,
                        alignment: Alignment::Middle,
                        ..Default::default()
                    }}
                />
            };
            dbg!(my_widget.wave_time);
        });
    }

    true
}

impl Widget for HudWaveTimerWidget {}

// Finally we need to let the core widget context know about our new widget!
pub fn hud_startup(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    let camera_entity = commands
        .spawn((Camera2dBundle::default(), CameraUIKayak))
        .id();
    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_system(
        HudWaveTimerWidget::default().get_name(),
        widget_update::<HudWaveTimerWidget, EmptyState>,
        hud_wave_timer_render,
    );
    widget_context.add_widget_data::<HudWaveTimerWidget, EmptyState>();

    let parent_id = None;

    commands.spawn(HudWaveTimerUpdate {
        timer: Timer::new(Duration::from_millis(1000), TimerMode::Repeating),
    });

    rsx! {
        <KayakAppBundle>
           <HudWaveTimerBundle props={HudWaveTimerWidget { wave_time: 0, }}></HudWaveTimerBundle>
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
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
