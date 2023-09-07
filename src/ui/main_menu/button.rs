use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

use crate::ui::main_menu::action::handle_button_hovering;

use super::assets::ImageAssets;

#[derive(Default, Clone, PartialEq, Component)]
pub struct MainMenuButton {
    pub text: String,
}

impl Widget for MainMenuButton {}

#[derive(Bundle)]
pub struct MainMenuButtonBundle {
    pub button: MainMenuButton,
    pub computed_styles: ComputedStyles,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for MainMenuButtonBundle {
    fn default() -> Self {
        Self {
            button: Default::default(),
            computed_styles: ComputedStyles(KStyle {
                position_type: KPositionType::ParentDirected.into(),
                //bottom: Units::Pixels(30.0).into(),
                bottom: Units::Stretch(1.0).into(),
                cursor: KCursorIcon(CursorIcon::Hand).into(),
                ..Default::default()
            }),
            on_event: OnEvent::default(),
            widget_name: MainMenuButton::default().get_name(),
        }
    }
}

pub fn main_menu_button_render(
    In(entity): In<Entity>,
    widget_context: Res<KayakWidgetContext>,
    mut commands: Commands,
    menu_button_query: Query<&MainMenuButton>,
    state_query: Query<&ButtonState>,
    images: Res<ImageAssets>,
) -> bool {
    let state_entity =
        widget_context.use_state(&mut commands, entity, ButtonState { hovering: false });

    let button_text = menu_button_query.get(entity).unwrap().text.clone();

    let on_event = OnEvent::new(
        move |In(_entity): In<Entity>,
              mut event: ResMut<KEvent>,
              mut query: Query<&mut ButtonState>| {
            if let Ok(mut button) = query.get_mut(state_entity) {
                match event.event_type {
                    EventType::MouseIn(..) => {
                        event.stop_propagation();
                        button.hovering = true;
                    }
                    EventType::MouseOut(..) => {
                        button.hovering = false;
                    }
                    _ => {}
                }
            }
        },
    );

    if let Ok(button_state) = state_query.get(state_entity) {
        let parent_id = Some(entity);
        rsx! {
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: handle_button_hovering(button_state, images),
                    border: Edge::all(10.0),
                }}
                styles={KStyle {
                    width: Units::Stretch(1.0).into(),
                    height: Units::Pixels(40.0).into(),
                    bottom: Units::Pixels(30.0).into(),
                    left: Units::Pixels(50.0).into(),
                    right: Units::Pixels(50.0).into(),
                    ..KStyle::default()
                }}
                on_event={on_event}
            >
                <TextWidgetBundle
                    styles={KStyle {
                        top: Units::Stretch(1.0).into(),
                        bottom: Units::Stretch(1.0).into(),
                        ..Default::default()
                    }}
                    text={TextProps {
                        alignment: Alignment::Middle,
                        content: button_text,
                        size: 28.0,
                        ..Default::default()
                    }}
                />
            </NinePatchBundle>
        };
    }
    true
}
