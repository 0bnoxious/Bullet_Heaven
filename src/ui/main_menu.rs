use bevy::prelude::*;
use kayak_ui::{prelude::*, widgets::*};

use crate::{
    game::state::{GameState, STARTING_GAME_STATE},
    ui::main_menu::{
        action::{handle_click_main_menu_exit, handle_click_main_menu_new_game},
        button::{MainMenuButton, MainMenuButtonBundle},
    },
};

use self::assets::ImageAssets;

pub mod action;
pub mod assets;
pub mod background;
pub mod button;

const BUTTON_TEXT_NEW_GAME: &str = "New Game";
const BUTTON_TEXT_SETTINGS: &str = "Settings";
const BUTTON_TEXT_EXIT_GAME: &str = "Exit Game";
const MENU_BACKGROUND_PIXEL: f32 = 1024.;
const MENU_SIZE_PIXEL: f32 = 512.;

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
    //pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}
impl Default for MainMenuBundle {
    fn default() -> Self {
        Self {
            props: MainMenuProps::default(),
            computed_styles: ComputedStyles(KStyle {
                position_type: KPositionType::SelfDirected.into(),
                width: StyleProp::Value(Units::Percentage(60.)),
                height: StyleProp::Value(Units::Percentage(60.)),
                top: Units::Percentage(20.).into(),
                left: Units::Percentage(20.).into(),
                ..Default::default()
            }),

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
    game_state: Res<State<GameState>>,
    images_res: Res<ImageAssets>,
) -> bool {
    let parent_id = Some(entity);

    let state_entity =
        widget_context.use_state(&mut commands, entity, ButtonState { hovering: false });

    rsx! {
        <ElementBundle>
        {
            if game_state.get() == &GameState::MainMenu {
                constructor! {
                    <NinePatchBundle
                    nine_patch={NinePatch {
                        handle: images_res.background_panel.clone(),
                        border: Edge::all(0.),
                    }}
                    styles={KStyle {
                        position_type: KPositionType::ParentDirected.into(),
                        width: Units::Pixels(MENU_SIZE_PIXEL).into(),
                        height: Units::Pixels(MENU_SIZE_PIXEL).into(),
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
                            image={KImage(images_res.player.clone())}
                            styles={KStyle {
                                top: Units::Pixels(25.0).into(),
                                left: Units::Stretch(1.0).into(),
                                right: Units::Stretch(1.0).into(),
                                position_type: KPositionType::ParentDirected.into(),
                                width: StyleProp::Value(Units::Percentage(40.)),
                                height: StyleProp::Value(Units::Percentage(40.)),
                                ..KStyle::default()
                            }}
                        />
                        <KImageBundle
                            image={KImage(images_res.tug_o_war_logo.clone())}
                            styles={KStyle {
                                top: Units::Pixels(25.0).into(),
                                left: Units::Percentage(20.0).into(),
                                width: Units::Pixels(310.0).into(),
                                height: Units::Pixels(78.0).into(),
                                bottom: Units::Stretch(1.0).into(),
                                ..KStyle::default()
                            }}
                        />
                        <MainMenuButtonBundle
                            button={MainMenuButton { text: BUTTON_TEXT_NEW_GAME.into() }}
                            on_event={handle_click_main_menu_new_game()}
                        />
                        <MainMenuButtonBundle button={MainMenuButton { text: BUTTON_TEXT_SETTINGS.into() }} />
                        <MainMenuButtonBundle
                            button={MainMenuButton { text: BUTTON_TEXT_EXIT_GAME.into() }}
                            on_event={handle_click_main_menu_exit()}
                        />
                    </NinePatchBundle>
                }
            }
        }
        </ElementBundle>
    };

    // The boolean returned here tells kayak UI to update the tree. You can avoid tree updates by
    // returning false, but in practice this should be done rarely. As kayak diff's the tree and
    // will avoid tree updates if nothing has changed!
    true
}

pub fn on_game_state_change(
    game_state: Res<State<GameState>>,
    mut main_menu: Query<&mut MainMenuProps, Without<PreviousWidget>>,
) {
    if game_state.is_changed() {
        for mut main_menu in main_menu.iter_mut() {
            main_menu.game_state = game_state.get().clone();
        }
    }
}
