use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollectionApp;
use kayak_ui::{prelude::*, widgets::*, CameraUIKayak};

use crate::game::state::GameState;
use crate::ui::hud::{hud_render, HudBundle, HudProps};
// use crate::ui::main_menu::background::main_menu_background_render;
use crate::ui::main_menu::button::MainMenuButton;
use crate::ui::main_menu::{main_menu_render, MainMenuBundle, MainMenuProps};
use crate::ui::{
    hud::wave_timer::hud_wave_timer_render, main_menu::button::main_menu_button_render,
};

use self::hud::setup_hud;
use self::hud::wave_timer::update_hud_wave_timer_value;
use self::main_menu::assets::ImageAssets;
use self::main_menu::on_game_state_change;

pub mod hud;
pub mod main_menu;
pub mod setting;

#[derive(Debug, Clone, PartialEq, Eq, Component)]
pub enum Menu {
    Main,
    Settings,
}

pub struct KayakUiPlugin;

impl Plugin for KayakUiPlugin {
    fn build(&self, app: &mut App) {
        app.init_collection::<ImageAssets>()
            .add_plugins((KayakContextPlugin, KayakWidgets))
            .add_systems(Startup, setup_kayak_ui)
            .add_systems(Startup, setup_hud)
            .add_systems(Update, (update_hud_wave_timer_value, on_game_state_change));
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

    // Main Menu ##########################################################
    widget_context.add_widget_data::<MainMenuProps, EmptyState>();
    widget_context.add_widget_system(
        MainMenuProps::default().get_name(),
        widget_update::<MainMenuProps, EmptyState>,
        main_menu_render,
    );

    // Main Menu buttons
    widget_context.add_widget_data::<MainMenuButton, ButtonState>();
    widget_context.add_widget_system(
        MainMenuButton::default().get_name(),
        widget_update::<MainMenuButton, ButtonState>,
        main_menu_button_render,
    );

    // player hud ##########################################################
    widget_context.add_widget_data::<HudProps, EmptyState>();
    widget_context.add_widget_system(
        HudProps::default().get_name(),
        widget_update::<HudProps, EmptyState>,
        hud_render,
    );

    // wave timer
    widget_context.add_widget_data::<HudProps, EmptyState>();
    widget_context.add_widget_system(
        HudProps::default().get_name(),
        widget_update::<HudProps, EmptyState>,
        hud_wave_timer_render,
    );

    rsx! {
        <KayakAppBundle>
            <HudBundle>
            </HudBundle>
            <MainMenuBundle>
            </MainMenuBundle>
        </KayakAppBundle>
    };

    commands.spawn((widget_context, EventDispatcher::default()));
}
