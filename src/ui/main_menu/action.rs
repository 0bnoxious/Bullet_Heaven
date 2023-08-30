use bevy::{app::AppExit, prelude::*};
use kayak_ui::{prelude::*, widgets::ButtonState};

use super::assets::ImageAssets;

pub fn handle_click_main_menu_exit() -> OnEvent {
    OnEvent::new(
        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut exit: EventWriter<AppExit>| {
            if let EventType::Click(..) = event.event_type {
                exit.send(AppExit);
            }
        },
    )
}

pub fn handle_button_hovering(
    button_state: &ButtonState,
    images: Res<ImageAssets>,
) -> Handle<Image> {
    if button_state.hovering {
        images.button_hover.clone()
    } else {
        images.button.clone()
    }
}
