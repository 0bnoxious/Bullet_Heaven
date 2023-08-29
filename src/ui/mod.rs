use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::*, CameraUIKayak};

use crate::ui::{
    game_menu::button::{menu_button_render, MenuButton},
    hud::wave_timer::{hud_wave_timer_render, HudWaveTimerWidget},
};

use self::{game_menu::GameMenuPlugin, hud::setup_hud};

pub mod game_menu;
pub mod hud;
pub mod settings;

pub struct KayakUiPlugin;

impl Plugin for KayakUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((KayakContextPlugin, KayakWidgets))
            .add_systems(Startup, setup_kayak_ui.before(setup_hud));
    }
}

pub fn setup_kayak_ui(
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

    // game menu
    // widget_context.add_widget_data::<MenuButton, ButtonState>();
    // widget_context.add_widget_system(
    //     MenuButton::default().get_name(),
    //     widget_update::<MenuButton, ButtonState>,
    //     menu_button_render,
    // );

    // player hud
    widget_context.add_widget_data::<HudWaveTimerWidget, EmptyState>();
    widget_context.add_widget_system(
        HudWaveTimerWidget::default().get_name(),
        widget_update::<HudWaveTimerWidget, EmptyState>,
        hud_wave_timer_render,
    );

    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <TextWidgetBundle
                text={TextProps {
                    content: "Hello World".into(),
                    size: 20.0,
                    alignment: Alignment::Middle,
                    ..Default::default()
                }}
            />
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
    println!("kayak_ui widget context set!");
}
