use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use kayak_ui::{prelude::*, widgets::*, CameraUIKayak};

use crate::ui::main_menu::background::{
    menu_background_render, MenuBackground, MenuBackgroundBundle,
};
use crate::ui::main_menu::button::MainMenuButtonBundle;
use crate::ui::{
    hud::wave_timer::{hud_wave_timer_render, HudWaveTimerBundle, HudWaveTimerWidget},
    main_menu::button::{main_menu_button_render, MainMenuButton},
};

use self::hud::wave_timer::update_hud_wave_timer_value;
use self::main_menu::assets::ImageAssets;
use self::main_menu::button::PreloadResource;
use self::{hud::setup_hud, main_menu::setup_game_menu};

pub mod hud;
pub mod main_menu;
pub mod settings;

pub struct KayakUiPlugin;

impl Plugin for KayakUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>()
            .init_resource::<PreloadResource>()
            .add_plugins((KayakContextPlugin, KayakWidgets))
            .add_systems(Startup, (setup_kayak_ui, setup_hud, setup_game_menu))
            .add_systems(Update, update_hud_wave_timer_value);
    }
}

pub fn setup_kayak_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    let camera_entity = commands
        .spawn(Camera2dBundle::default())
        .insert(CameraUIKayak)
        .id();

    font_mapping.set_default(asset_server.load("roboto.kayak_font"));

    let mut widget_context = KayakRootContext::new(camera_entity);
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <HudWaveTimerBundle/>
            <MenuBackgroundBundle/>
            //<MenuButtonBundle/>
        </KayakAppBundle>
    };

    // Menu background
    widget_context.add_widget_data::<MenuBackground, EmptyState>();
    widget_context.add_widget_system(
        MenuBackground::default().get_name(),
        widget_update::<MenuBackground, EmptyState>,
        menu_background_render,
    );

    // Menu buttons
    widget_context.add_widget_data::<MainMenuButton, ButtonState>();
    widget_context.add_widget_system(
        MainMenuButton::default().get_name(),
        widget_update::<MainMenuButton, ButtonState>,
        main_menu_button_render,
    );

    // player hud
    widget_context.add_widget_data::<HudWaveTimerWidget, EmptyState>();
    widget_context.add_widget_system(
        HudWaveTimerWidget::default().get_name(),
        widget_update::<HudWaveTimerWidget, EmptyState>,
        hud_wave_timer_render,
    );

    commands.spawn((widget_context, EventDispatcher::default()));
}
