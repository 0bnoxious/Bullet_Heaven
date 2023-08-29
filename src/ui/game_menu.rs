use bevy::{app::AppExit, prelude::*};
use kayak_ui::prelude::{widgets::*, *};

use self::button::PreloadResource;
use crate::ui::game_menu::button::{MenuButton, MenuButtonBundle};

pub mod button;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PreloadResource>()
            .add_systems(Startup, setup_game_menu);
    }
}

pub fn setup_game_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut preload_resource: ResMut<PreloadResource>,
    mut widget_context: ResMut<KayakWidgetContext>,
) {
    println!("setting game menu...");
    // widget_context.add_widget_data::<MenuButton, ButtonState>();
    // widget_context.add_widget_system(
    //     MenuButton::default().get_name(),
    //     widget_update::<MenuButton, ButtonState>,
    //     menu_button_render,
    // );

    let panel1_image = asset_server.load("main_menu/panel1.png");
    let logo_image = asset_server.load("main_menu/logo.png");
    let kayak_image = asset_server.load("main_menu/kayak.png");
    let button_image = asset_server.load("main_menu/button.png");
    let button_image_hover = asset_server.load("main_menu/button-hover.png");

    preload_resource.images.extend(vec![
        panel1_image.clone(),
        logo_image.clone(),
        button_image,
        button_image_hover,
    ]);

    let handle_click_close = OnEvent::new(
        move |In(_entity): In<Entity>, event: ResMut<KEvent>, mut exit: EventWriter<AppExit>| {
            if let EventType::Click(..) = event.event_type {
                exit.send(AppExit);
            }
        },
    );

    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: panel1_image,
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
                    image={KImage(kayak_image)}
                    styles={KStyle {
                        width: Units::Pixels(310.0).into(),
                        height: Units::Pixels(104.0).into(),
                        top: Units::Pixels(25.0).into(),
                        bottom: Units::Pixels(25.0).into(),
                        ..KStyle::default()
                    }}
                />
                <KImageBundle
                    image={KImage(logo_image)}
                    styles={KStyle {
                        width: Units::Pixels(310.0).into(),
                        height: Units::Pixels(78.0).into(),
                        bottom: Units::Stretch(1.0).into(),
                        ..KStyle::default()
                    }}
                />
                <MenuButtonBundle button={MenuButton { text: "Play".into() }} />
                <MenuButtonBundle button={MenuButton { text: "Options".into() }} />
                <MenuButtonBundle
                    button={MenuButton { text: "Quit".into() }}
                    on_event={handle_click_close}
                />
            </NinePatchBundle>
        </KayakAppBundle>
    };
    println!("game menu set!");
}
