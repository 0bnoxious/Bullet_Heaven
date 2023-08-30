use bevy::{app::AppExit, prelude::*};
use kayak_ui::{
    prelude::*,
    widgets::{KImage, KImageBundle, NinePatch, NinePatchBundle},
};

use crate::ui::main_menu::{
    action::handle_click_main_menu_exit,
    button::{MainMenuButton, MainMenuButtonBundle},
    BUTTON_TEXT_EXIT_GAME, BUTTON_TEXT_NEW_GAME, BUTTON_TEXT_SETTINGS,
};

use super::{assets::ImageAssets, MainMenuProps};

#[derive(Bundle)]
pub struct MainMenuBackgroundBundle {
    pub styles: KStyle,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for MainMenuBackgroundBundle {
    fn default() -> Self {
        Self {
            styles: KStyle {
                ..Default::default()
            },
            on_event: OnEvent::default(),
            widget_name: MainMenuProps::default().get_name(),
            children: KChildren::default(),
        }
    }
}

pub fn main_menu_background_render(
    In(entity): In<Entity>,
    widget_context: ResMut<KayakWidgetContext>,
    mut commands: Commands,
    images: Res<ImageAssets>,
) -> bool {
    let parent_id = Some(entity);
    rsx! {
        <NinePatchBundle
            nine_patch={NinePatch {
                handle: images.background_panel.clone(),
                border: Edge::all(25.0),
            }}
            styles={KStyle {
                width: Units::Pixels(350.0).into(),
                height: Units::Pixels(512.0).into(),
                left: Units::Stretch(1.0).into(),
                right: Units::Stretch(1.0).into(),
                top: Units::Stretch(1.0).into(),
                bottom: Units::Stretch(1.0).into(),
                padding: Edge::new(
                    Units::Pixels(20.0),
                    Units::Pixels(20.0),
                    Units::Pixels(50.0),
                    Units::Pixels(20.0),
                ).into(),
                ..KStyle::default()
            }}
        >
            <KImageBundle
                image={KImage(images.player.clone())}
                styles={KStyle {
                    width: Units::Pixels(310.0).into(),
                    height: Units::Pixels(104.0).into(),
                    top: Units::Pixels(25.0).into(),
                    bottom: Units::Pixels(25.0).into(),
                    ..KStyle::default()
                }}
            />
            <KImageBundle
                image={KImage(images.tug_o_war_logo.clone())}
                styles={KStyle {
                    width: Units::Pixels(310.0).into(),
                    height: Units::Pixels(78.0).into(),
                    bottom: Units::Stretch(1.0).into(),
                    ..KStyle::default()
                }}
            />
            <MainMenuButtonBundle button={MainMenuButton { text: BUTTON_TEXT_NEW_GAME.into() }} />
            <MainMenuButtonBundle button={MainMenuButton { text: BUTTON_TEXT_SETTINGS.into() }} />
            <MainMenuButtonBundle
                button={MainMenuButton { text: BUTTON_TEXT_EXIT_GAME.into() }}
                on_event={handle_click_main_menu_exit()}
            />
        </NinePatchBundle>
    };

    true
}
