use bevy::{app::AppExit, prelude::*};
use kayak_ui::prelude::*;

use crate::{
    global::{GameState, STARTING_GAME_STATE},
    ui::main_menu::{background::MainMenuBackgroundBundle, button::MainMenuButtonBundle},
};

pub mod action;
pub mod assets;
pub mod background;
pub mod button;

const BUTTON_TEXT_NEW_GAME: &str = "New Game";
const BUTTON_TEXT_SETTINGS: &str = "Settings";
const BUTTON_TEXT_EXIT_GAME: &str = "Exit Game";

#[derive(Clone, PartialEq, Component)]
pub struct MainMenuProps {
    pub game_state: GameState,
    pub new_game_button_text: String,
    pub settings_button_text: String,
    pub exit_button_text: String,
}

impl Default for MainMenuProps {
    fn default() -> Self {
        Self {
            game_state: STARTING_GAME_STATE,
            new_game_button_text: BUTTON_TEXT_NEW_GAME.to_string(),
            settings_button_text: BUTTON_TEXT_SETTINGS.to_string(),
            exit_button_text: BUTTON_TEXT_EXIT_GAME.to_string(),
        }
    }
}

impl Widget for MainMenuProps {}

#[derive(Bundle)]
pub struct MainMenuBundle {
    pub props: MainMenuProps,
    pub styles: KStyle,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for MainMenuBundle {
    fn default() -> Self {
        Self {
            props: MainMenuProps::default(),
            styles: KStyle::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: MainMenuProps::default().get_name(),
        }
    }
}

pub fn main_menu_render(
    In(entity): In<Entity>,
    widget_context: ResMut<KayakWidgetContext>,
    mut commands: Commands,
) -> bool {
    let parent_id = Some(entity);
    rsx! {
        <MainMenuBackgroundBundle>
            <MainMenuButtonBundle/>
        </MainMenuBackgroundBundle>

    };

    // The boolean returned here tells kayak UI to update the tree. You can avoid tree updates by
    // returning false, but in practice this should be done rarely. As kayak diff's the tree and
    // will avoid tree updates if nothing has changed!
    true
}
