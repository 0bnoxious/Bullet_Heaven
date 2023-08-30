use bevy::{app::AppExit, prelude::*};
use kayak_ui::{
    prelude::*,
    widgets::{KImage, KImageBundle, NinePatch, NinePatchBundle},
};

use crate::ui::main_menu::button::{MainMenuButton, MainMenuButtonBundle};

use super::assets::ImageAssets;

#[derive(Default, Clone, PartialEq, Component)]
pub struct MainMenuBackgroundProps {}

impl Widget for MainMenuBackgroundProps {}

#[derive(Bundle)]
pub struct MainMenuBackgroundBundle {
    pub styles: KStyle,
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
            widget_name: MainMenuBackgroundProps::default().get_name(),
        }
    }
}

pub fn main_menu_background_render(
    In(entity): In<Entity>,
    widget_context: ResMut<KayakWidgetContext>,
    mut commands: Commands,
    images: Res<ImageAssets>,
) -> bool {
    let handle_click_close = OnEvent::new(
        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut exit: EventWriter<AppExit>| {
            if let EventType::Click(..) = event.event_type {
                exit.send(AppExit);
            }
        },
    );

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
            <MainMenuButtonBundle button={MainMenuButton { text: "Play".into() }} />
            <MainMenuButtonBundle button={MainMenuButton { text: "Options".into() }} />
            <MainMenuButtonBundle
                button={MainMenuButton { text: "Quit".into() }}
                on_event={handle_click_close}
            />
        </NinePatchBundle>
    };

    true
}
